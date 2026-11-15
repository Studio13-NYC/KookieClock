"""Original 3D wooden-toy artwork. Run with Blender 5.1 --background --factory-startup.

Produces transparent PNG components for the native Rust renderer. It never opens or
changes a user's Blender scene. No image-generation services or downloaded assets.
"""
import bpy, bmesh, math, re, sys
from pathlib import Path
from mathutils import Vector

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'assets'/'woodland'
OUT.mkdir(parents=True,exist_ok=True)
(ROOT/'artifacts').mkdir(parents=True,exist_ok=True)
args=sys.argv[sys.argv.index('--')+1:] if '--' in sys.argv else []
ONLY=args[0] if args else 'all'

def linear(v):
    return v/12.92 if v<=0.04045 else ((v+0.055)/1.055)**2.4
def rgb(value):
    return tuple(linear(((value>>shift)&255)/255) for shift in (16,8,0))+(1,)

def material(name,color,wood=False,paint=False):
    m=bpy.data.materials.new(name);m.use_nodes=True
    n=m.node_tree.nodes;l=m.node_tree.links
    p=next(x for x in n if x.type=='BSDF_PRINCIPLED')
    p.inputs['Base Color'].default_value=rgb(color)
    p.inputs['Roughness'].default_value=.48 if wood else .58
    p.inputs['Coat Weight'].default_value=.09 if wood else .035
    p.inputs['Coat Roughness'].default_value=.42
    if wood or paint:
        tex=n.new('ShaderNodeTexCoord')
        stretch=n.new('ShaderNodeVectorMath');stretch.operation='MULTIPLY'
        stretch.inputs[1].default_value=(3.,8.,.22)
        l.new(tex.outputs['Generated'],stretch.inputs[0])
        noise=n.new('ShaderNodeTexNoise');noise.inputs['Scale'].default_value=5.
        noise.inputs['Detail'].default_value=3.;noise.inputs['Roughness'].default_value=.72
        l.new(stretch.outputs['Vector'],noise.inputs['Vector'])
        ramp=n.new('ShaderNodeValToRGB')
        base=rgb(color);strength=.3 if wood else .14
        ramp.color_ramp.elements[0].position=.12
        ramp.color_ramp.elements[1].position=.87
        ramp.color_ramp.elements[0].color=tuple(c*(1-strength) for c in base[:3])+(1,)
        ramp.color_ramp.elements[1].color=tuple(min(1,c*(1+strength*.55)) for c in base[:3])+(1,)
        l.new(noise.outputs['Fac'],ramp.inputs[0]);l.new(ramp.outputs[0],p.inputs['Base Color'])
        grain=n.new('ShaderNodeTexNoise');grain.inputs['Scale'].default_value=48.
        grain.inputs['Detail'].default_value=2.
        l.new(stretch.outputs[0],grain.inputs['Vector'])
        bump=n.new('ShaderNodeBump');bump.inputs['Strength'].default_value=.16 if wood else .13
        bump.inputs['Distance'].default_value=.013 if wood else .009
        l.new(grain.outputs['Fac'],bump.inputs['Height']);l.new(bump.outputs[0],p.inputs['Normal'])
    return m

wood=material('Natural maple / longitudinal grain',0xd9b98b,True)
endwood=material('Warm end grain',0xc19b68,True)
cream=material('Warm ivory milk paint',0xf5eedf,paint=True)
teal=material('Turquoise milk paint',0x008e9b,paint=True)
sage=material('Sunflower shutters',0xe9ae19,paint=True)
dark=material('Deep blue-green recess',0x263f42,paint=True)
ink=material('Printed charcoal',0x303c3a)
ochre=material('Golden yellow paint',0xf3b51a,paint=True)
white=material('Eye paint',0xf7edda)
black=material('Black eye paint',0x172c30)
pink=material('Coral cheek paint',0xe35b3f,paint=True)
coral=material('Tomato red milk paint',0xd93f2b,paint=True)
leaf=material('Leaf green milk paint',0x6c9d24,paint=True)

def finish(o,mat,bevel=0,smooth=True):
    o.data.materials.append(mat)
    if bevel:
        b=o.modifiers.new('Soft sanded edges','BEVEL');b.width=bevel;b.segments=4
        b.affect='EDGES'
        o.modifiers.new('Weighted corner normals','WEIGHTED_NORMAL')
    if smooth:
        for f in o.data.polygons:f.use_smooth=True
    return o
def cube(name,loc,dim,mat,bevel=.035):
    bpy.ops.mesh.primitive_cube_add(size=1,location=loc);o=bpy.context.object;o.name=name
    o.dimensions=dim;bpy.ops.object.transform_apply(location=False,rotation=False,scale=True)
    return finish(o,mat,bevel,False)
def sphere(name,loc,scale,mat):
    bpy.ops.mesh.primitive_uv_sphere_add(segments=40,ring_count=24,location=loc)
    o=bpy.context.object;o.name=name;o.scale=scale
    bpy.ops.object.transform_apply(location=False,rotation=False,scale=True)
    return finish(o,mat)
def cylinder(name,loc,radius,depth,mat,front=False,bevel=.025):
    bpy.ops.mesh.primitive_cylinder_add(vertices=96,radius=radius,depth=depth,location=loc)
    o=bpy.context.object;o.name=name
    if front:o.rotation_euler[0]=math.pi/2
    return finish(o,mat,bevel)
def cone(name,loc,radius,depth,mat,rotation=(0,0,0)):
    bpy.ops.mesh.primitive_cone_add(vertices=64,radius1=radius,radius2=.025,depth=depth,location=loc,rotation=rotation)
    o=bpy.context.object;o.name=name;return finish(o,mat,.022)
def beam(name,p1,p2,width,depth,mat):
    v=Vector(p2)-Vector(p1)
    o=cube(name,(Vector(p1)+Vector(p2))/2,(width,depth,v.length),mat,width*.3)
    # All toy beams lie in the front-facing XZ plane: keep their depth on Y.
    o.rotation_euler=(0,math.atan2(v.x,v.z),0);return o
def extrude(name,points,front,back,mat,bevel=.03):
    n=len(points);verts=[(x,y,z)for y in (front,back)for x,z in points]
    faces=[tuple(range(n-1,-1,-1)),tuple(range(n,2*n))]
    faces += [(i,(i+1)%n,(i+1)%n+n,i+n)for i in range(n)]
    mesh=bpy.data.meshes.new(name);mesh.from_pydata(verts,[],faces);mesh.update()
    bm=bmesh.new();bm.from_mesh(mesh);bmesh.ops.recalc_face_normals(bm,faces=list(bm.faces));bm.to_mesh(mesh);bm.free()
    o=bpy.data.objects.new(name,mesh);bpy.context.collection.objects.link(o);return finish(o,mat,bevel,False)
def arch(cx,bottom,shoulder,radius):
    return [(cx-radius,bottom),(cx+radius,bottom)]+[(cx+math.cos(a)*radius,shoulder+math.sin(a)*radius)for a in [i*math.pi/32 for i in range(33)]]

def clear():
    for o in list(bpy.data.objects):bpy.data.objects.remove(o,do_unlink=True)
def setup(w,h,center_z,scale):
    sc=bpy.context.scene
    sc.render.engine='CYCLES';sc.cycles.samples=40;sc.cycles.use_denoising=True
    sc.render.resolution_x=w;sc.render.resolution_y=h;sc.render.resolution_percentage=100
    sc.render.image_settings.file_format='PNG';sc.render.image_settings.color_mode='RGBA'
    sc.render.image_settings.color_depth='8';sc.render.film_transparent=True
    sc.world.color=(.35,.35,.35)
    sc.view_settings.view_transform='AgX'
    sc.view_settings.look='AgX - Medium High Contrast'
    sc.view_settings.exposure=.65
    bpy.ops.object.camera_add(location=(0,-18,center_z));cam=bpy.context.object
    cam.rotation_euler=(math.pi/2,0,0);cam.data.type='ORTHO'
    # Blender fits the longer image dimension; callers specify vertical coverage.
    cam.data.ortho_scale=scale*max(1.,w/h);sc.camera=cam
    for name,loc,power,size,col in [('Large softbox',(-3,-5,8),650,4.5,(1,.91,.8)),('Soft fill',(4,-2,4),220,5.,(.85,.93,1)),('Top edge',(0,2,7),400,3.,(1,.97,.88))]:
        bpy.ops.object.light_add(type='AREA',location=loc);o=bpy.context.object;o.name=name
        o.data.energy=power;o.data.shape='DISK';o.data.size=size;o.data.color=col
        o.rotation_euler=(Vector((0,0,center_z))-o.location).to_track_quat('-Z','Y').to_euler()
    return sc
def render(name):
    s=bpy.context.scene;s.render.filepath=str(OUT/(name+'.png'))
    bpy.ops.render.render(write_still=True)
    print('ASSET COMPLETE',name,flush=True)

def flower(name,x,z,radius=.16,front=-.205):
    # Thin, rounded paint inlays retain the grain and physical toy lighting.
    for i in range(6):
        a=i*math.tau/6
        sphere(name+' petal',(x+math.sin(a)*radius*.62,front,z+math.cos(a)*radius*.62),
               (radius*.43,.008,radius*.43),coral)
    sphere(name+' golden center',(x,front-.012,z),(radius*.30,.008,radius*.30),ochre)

def sprig(x,z):
    beam('Painted flower stem',(x,-.20,z-.44),(x,-.20,z-.10),.022,.012,leaf)
    for dz,side in [(-.33,-1),(-.23,1)]:
        o=sphere('Painted leaf',(x+side*.063,-.211,z+dz),(.093,.009,.034),leaf)
        o.rotation_euler[1]=-side*.48
    flower('Folk flower',x,z,.145)


def clock_body():
    clear();setup(800,1150,2.875,5.75)
    body=extrude('Maple house',[(-1.27,1.23),(1.27,1.23),(1.27,4.45),(0,5.49),(-1.27,4.45)],-.12,.34,wood,.075)
    # A true routed recess, not a painted dark shape.
    cutter=extrude('Temporary arch cutter',arch(0,3.53,4.14,.57),-.6,.17,wood,0)
    mod=body.modifiers.new('Routed bird alcove','BOOLEAN');mod.operation='DIFFERENCE';mod.object=cutter
    bpy.context.view_layer.objects.active=body
    bpy.ops.object.modifier_move_to_index(modifier=mod.name,index=0)
    bpy.ops.object.modifier_apply(modifier=mod.name)
    bpy.data.objects.remove(cutter,do_unlink=True)
    extrude('Dark painted back of alcove',arch(0,3.54,4.14,.568),.14,.16,dark,.01)
    extrude('Joined painted roof',[(-1.65,4.30),(0,5.65),(1.65,4.30),(1.50,4.16),(0,5.39),(-1.50,4.16)],-.44,.28,teal,.055)
    cube('Painted turquoise base ledge',(0,-.05,1.25),(2.82,.72,.18),teal,.065)
    for x in [-1.08,-.81,-.54,-.27,0,.27,.54,.81,1.08]:
        sphere('Golden base dots',(x,-.414,1.25),(.032,.006,.032),ochre)
    flower('Gable rosette',0,5.02,.18)
    sprig(-.94,4.05)
    sprig(.94,4.05)
    cube('Bird landing ledge',(0,-.27,3.53),(1.47,.57,.12),wood,.045)
    # Turned dial, with smooth bevel and a low ivory face.
    cylinder('Turned dial surround',(0,-.22,2.48),1.10,.17,coral,True,.045)
    cylinder('Maple dial bevel',(0,-.32,2.48),1.055,.16,wood,True,.055)
    cylinder('Ivory dial',(0,-.414,2.48),.975,.045,cream,True,.018)
    font=bpy.data.fonts.load('C:/Windows/Fonts/bahnschrift.ttf')
    for i in range(60):
        a=i*math.tau/60
        r=.895;length=.055 if i%5==0 else .023
        o=cube('Printed minute index',(math.sin(a)*r,-.440,2.48+math.cos(a)*r),(.018 if i%5==0 else .006,.002,length),ink,.002)
        o.rotation_euler[1]=a
    for n in range(1,13):
        a=n*math.tau/12;r=.735
        curve=bpy.data.curves.new('Printed numeral','FONT');curve.body=str(n);curve.font=font
        curve.align_x='CENTER';curve.align_y='CENTER';curve.size=.217;curve.extrude=.0003;curve.space_character=1.04
        o=bpy.data.objects.new('Dial '+str(n),curve);bpy.context.collection.objects.link(o)
        o.location=(math.sin(a)*r,-.443,2.48+math.cos(a)*r);o.rotation_euler=(math.pi/2,0,0);curve.materials.append(ink)
    # The thin cords and weights are real turned wood.
    for x in [-.88,.88]:
        beam('Weight cord',(x,.08,1.27),(x,.08,.61),.019,.019,endwood)
        cylinder('Painted turned weight',(x,.08,.64),.085,.32,coral if x<0 else teal,False,.06)
    render('house')
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT/'clock-source.blend'))

def pendulum():
    clear();setup(256,512,.68,2.56)
    beam('Maple pendulum shaft',(0,0,1.8),(0,0,-.15),.045,.045,endwood)
    cylinder('Turned pendulum bob',(0,-.04,.015),.245,.15,endwood,True,.04)
    cylinder('Ochre inset',(0,-.13,.015),.203,.025,ochre,True,.015)
    flower('Pendulum rosette',0,.015,.13,-.15)
    render('pendulum')

def doors():
    clear();setup(440,290,4.125,1.45)
    # These panels share the case's real world coordinates. Each leaf rotates
    # about a vertical hinge, so thickness and foreshortening remain physical.
    seams=material('Door joinery',0xc88b15,paint=True)
    pivots=[]
    for side in [-1,1]:
        first=set(bpy.data.objects)
        left=[(-.561,3.545),(-.009,3.545),(-.009,4.701)]
        left += [(math.cos(math.pi/2+i*math.pi/64)*.561,4.14+math.sin(math.pi/2+i*math.pi/64)*.561)for i in range(1,33)]
        points=left if side==-1 else [(-x,z)for x,z in reversed(left)]
        extrude('Sanded sunflower door',points,-.255,-.17,sage,.018)
        cube('Fine vertical join',(side*.28,-.256,3.98),(.007,.004,.84),seams,.002)
        sphere('Small coral door knob',(side*.09,-.29,3.87),(.027,.032,.027),coral)
        for z in [3.76,3.89,4.02]:
            sphere('Painted shutter dot',(side*.39,-.278,z),(.024,.007,.024),cream)
        o=sphere('Shutter leaf',(side*.30,-.278,4.30),(.115,.008,.047),teal)
        o.rotation_euler[1]=side*.6
        parts=set(bpy.data.objects)-first
        pivot=bpy.data.objects.new('Left hinge pivot' if side==-1 else 'Right hinge pivot',None)
        bpy.context.collection.objects.link(pivot);pivot.location=(side*.57,-.17,3.54)
        bpy.context.view_layer.update()
        for o in parts:
            matrix=o.matrix_world.copy();o.parent=pivot;o.matrix_world=matrix
        pivots.append((side,pivot))
        for z in [3.72,4.10]:cylinder('Small hinge pin',(side*.57,-.19,z),.027,.13,endwood,False,.01)
    for frame in range(17):
        for side,pivot in pivots:pivot.rotation_euler[2]=side*math.radians(112.*frame/16)
        render('doors-%02d'%frame)

def joint(name,parts,pivot):
    parent=bpy.data.objects.new(name,None);bpy.context.collection.objects.link(parent)
    parent.location=pivot;bpy.context.view_layer.update()
    for o in parts:
        if o.parent in parts: continue
        matrix=o.matrix_world.copy();o.parent=parent;o.matrix_world=matrix
    return parent

def curve_beam(name,points,radius,mat):
    c=bpy.data.curves.new(name,'CURVE');c.dimensions='3D';c.bevel_depth=radius;c.bevel_resolution=3
    sp=c.splines.new('BEZIER');sp.bezier_points.add(len(points)-1)
    for bp,p in zip(sp.bezier_points,points):bp.co=p;bp.handle_left_type='AUTO';bp.handle_right_type='AUTO'
    o=bpy.data.objects.new(name,c);bpy.context.collection.objects.link(o);o.data.materials.append(mat);return o

def character(index,color,accent):
    clear();setup(192,192,1.0,2.8)
    bpy.context.scene.cycles.samples=16
    body=material('Character milk paint',color,paint=True);mark=material('Contrasting milk paint',accent,paint=True)
    before=set(bpy.data.objects);rig={}
    def group(key,start,pivot):
        rig[key]=joint(key,set(bpy.data.objects)-start,pivot);return rig[key]
    def foot(x,y=0,size=.15):
        return sphere('Carved foot',(x,y,.09),(size,.23,.09),ochre if index<12 else wood)
    def head(center,shape,ears=None,beak=None,patch=False):
        start=set(bpy.data.objects);x,y,z=center;rx,ry,rz=shape
        sphere('Head',center,shape,body)
        if ears:
            for side in [-1,1]:
                ex=x+side*rx*.8;ez=z+rz*.85
                e=set(bpy.data.objects)
                if ears=='long':
                    sphere('Long wooden ear',(ex,y,ez+.27),(.09,.10,.37),body)
                    sphere('Ear paint',(ex,y-.095,ez+.29),(.043,.014,.26),pink)
                elif ears=='point':cone('Triangular ear',(ex,y,ez+.06),.13,.28,body)
                else:
                    radius=.21 if index==17 else .12
                    sphere('Round wooden ear',(ex,y,ez), (radius,.10,radius),body)
                    sphere('Ear paint',(ex,y-.09,ez), (radius*.6,.018,radius*.6),mark)
                group('ear'+str(side),e,(ex,y,ez-.05))
        for side in [-1,1]:
            ex=x+side*rx*.42
            if patch:sphere('Face patch',(ex,y-ry*.94,z+.035),(rx*.43,.024,rz*.65),cream if index!=22 else black)
            sphere('Eye ivory',(ex,y-ry,z+.045),(.092,.025,.106),white)
            sphere('Eye pupil',(ex+.008,y-ry-.027,z+.045),(.052,.018,.067),black)
            sphere('Eye glint',(ex-.01,y-ry-.045,z+.07),(.016,.006,.018),white)
        if beak=='duck':sphere('Broad flat bill',(x,y-ry-.12,z-.14),(.20,.23,.065),mark)
        elif beak=='toucan':
            sphere('Oversized curved bill',(x+.30,y-ry-.08,z-.08),(.43,.20,.17),ochre)
            sphere('Red bill tip',(x+.62,y-ry-.09,z-.08),(.12,.17,.12),coral)
        elif beak:
            o=cone('Pointed beak',(x,y-ry-.08,z-.12),.085,.29 if index==9 else .18,ochre,(math.pi/2,0,0))
        elif index!=18:
            sphere('Carved muzzle',(x,y-ry-.03,z-.13),(.17,.10,.11),cream)
            sphere('Nose',(x,y-ry-.125,z-.1),(.049,.03,.035),black)
        for side in [-1,1]:sphere('Cheek',(x+side*rx*.70,y-ry*.84,z-.12),(.045,.012,.026),pink)
        group('head',start,(x,y,z-rz*.65))
    def wing(side,z=.65,length=.32):
        start=set(bpy.data.objects)
        sphere('Hinged wing',(side*.35,0,z-.1),(.115,.22,length),mark)
        group('wing'+str(side),start,(side*.32,0,z+.17))
    def arms(z=.65,width=.34):
        for side in [-1,1]:
            start=set(bpy.data.objects)
            sphere('Wooden arm',(side*width,-.08,z-.1),(.105,.14,.23),mark)
            group('arm'+str(side),start,(side*width,0,z+.12))
    def tail(points,radius=.10):
        start=set(bpy.data.objects);curve_beam('Carved tail',points,radius,mark)
        group('tail',start,points[0])
    def dots(z,y=-.3):
        for row in range(2):
            for col in range(3):sphere('Milk paint dots',((col-1)*.11,y,z-row*.11),(.03,.008,.03),cream)
    # Deliberately different construction families: balls, blocks, cones,
    # barrels, low pull-toy animals and long-legged four-legged figures.
    if index==0:
        cone('Tall cuckoo cone',(0,0,.70),.35,1.12,body)
        head((0,0,1.36),(.28,.25,.28),beak='bird');wing(-1,.86);wing(1,.86)
        tail([(0,.20,.35),(.20,.36,.55),(.28,.38,.78)],.09);dots(.7,-.23)
    elif index==1:
        sphere('Round robin',(0,0,.67),(.48,.35,.48),body)
        sphere('Red breast',(0,-.32,.57),(.31,.055,.31),coral)
        head((0,-.015,1.10),(.29,.27,.26),beak='bird');wing(-1,.69,.20);wing(1,.69,.20)
    elif index==2:
        cube('Chunky owl block',(0,0,.68),(.89,.55,1.13),body,.16)
        head((0,0,1.20),(.45,.29,.32),ears='point',beak='bird',patch=True)
        wing(-1,.70,.27);wing(1,.70,.27);dots(.66,-.287)
    elif index==3:
        sphere('Slender bluebird',(0,0,.64),(.28,.28,.47),body)
        head((0,-.03,1.22),(.28,.25,.28),beak='bird');wing(-1,.8,.38);wing(1,.8,.38)
        tail([(0,.2,.35),(.32,.30,.39),(.56,.28,.65)],.09)
    elif index==4:
        cylinder('Tall cockatoo barrel',(0,0,.72),.27,1.15,body,False,.10)
        head((0,0,1.43),(.33,.27,.33),beak='bird')
        start=set(bpy.data.objects)
        for i in range(3):
            o=sphere('Crest slat',((i-1)*.12,0,1.83+(.06 if i==1 else 0)),(.065,.11,.24),coral);o.rotation_euler[1]=(i-1)*.25
        group('crest',start,(0,0,1.65));wing(-1,.95,.42);wing(1,.95,.42);dots(.74,-.275)
    elif index==5:
        sphere('Penguin bowling pin',(0,0,.74),(.39,.31,.61),body)
        sphere('Ivory bib',(0,-.288,.67),(.28,.045,.46),cream)
        head((0,0,1.35),(.29,.27,.29),beak='bird',patch=True);wing(-1,.88,.40);wing(1,.88,.40)
    elif index==6:
        sphere('Toucan barrel',(0,0,.58),(.33,.29,.41),body)
        head((-.12,0,1.17),(.34,.29,.34),beak='toucan');wing(-1,.68,.25);wing(1,.68,.25)
        tail([(0,.19,.39),(-.25,.3,.25),(-.43,.33,.3)],.11)
    elif index==7:
        sphere('Low duck pear',(0,.05,.46),(.50,.37,.30),body)
        sphere('Curved duck neck',(-.10,-.07,.79),(.20,.21,.32),body)
        head((-.1,-.07,1.06),(.29,.26,.28),beak='duck');wing(-1,.50,.18);wing(1,.50,.18)
        tail([(0,.23,.44),(.36,.29,.58),(.49,.26,.69)],.09)
    elif index==8:
        cone('Little finch spindle',(0,0,.55),.30,.78,body)
        head((0,0,1.04),(.26,.24,.26),beak='bird');wing(-1,.61,.22);wing(1,.61,.22)
        tail([(0,.1,.32),(.27,.22,.32),(.46,.25,.51)],.075)
    elif index==9:
        sphere('Woodpecker spindle',(0,0,.78),(.24,.26,.57),body)
        head((0,-.02,1.43),(.27,.25,.3),beak='bird')
        start=set(bpy.data.objects)
        extrude('Red triangular crest',[(-.20,1.65),(.20,1.65),(.12,1.98)],-.10,.12,coral,.025)
        group('crest',start,(0,0,1.65));wing(-1,.80,.33);wing(1,.80,.33)
    elif index==10:
        sphere('Dove teardrop',(0,.03,.59),(.40,.31,.38),body)
        sphere('Slender neck',(0,-.04,.95),(.15,.17,.31),body)
        head((0,-.04,1.26),(.235,.22,.23),beak='bird');wing(-1,.65,.32);wing(1,.65,.32)
    elif index==11:
        start=set(bpy.data.objects)
        for i in range(7):
            a=(i-3)*.32
            o=sphere('Fan feather',(math.sin(a)*.60,.22,.68+math.cos(a)*.43),(.13,.075,.46),teal if i%2 else leaf);o.rotation_euler[1]=a
            sphere('Golden feather eye',(math.sin(a)*.82,.135,.68+math.cos(a)*.72),(.065,.022,.09),ochre)
        group('fan',start,(0,.23,.28))
        cone('Peacock pedestal',(0,-.08,.44),.31,.65,body)
        head((0,-.10,.94),(.29,.26,.29),beak='bird');wing(-1,.50,.21);wing(1,.50,.21)
    elif index in (12,19):
        sphere('Four legged body',(0,.02,.62),(.47,.28,.28),body)
        for side in [-1,1]:
            for front in [-1,1]:
                st=set(bpy.data.objects)
                beam('Wooden leg',(side*.32,front*.18,.62),(side*.32,front*.18,.10),.105,.105,body)
                foot(side*.32,front*.18,.09);group('leg'+str(side)+str(front),st,(side*.32,front*.18,.60))
        sphere('Raised neck',(-.22,0,.96),(.19,.20,.38),body)
        head((-.25,-.04,1.30),(.28,.25,.27),ears='point')
        if index==12:tail([(.33,.16,.63),(.63,.18,.85),(.67,.15,1.15)],.17)
        else:
            st=set(bpy.data.objects)
            for side in [-1,1]:
                x=-.25+side*.20
                beam('Antler',(x,.03,1.51),(x+side*.08,.03,1.95),.035,.035,endwood)
                beam('Antler fork',(x+side*.05,.03,1.78),(x+side*.20,.03,1.88),.03,.03,endwood)
            group('antlers',st,(-.25,0,1.48))
    elif index==13:
        sphere('Rabbit haunch',(0,.07,.55),(.39,.33,.40),body)
        head((0,-.08,1.11),(.29,.25,.30),ears='long');arms(.67,.28)
        for side in [-1,1]:sphere('Long back foot',(side*.27,-.03,.12),(.15,.32,.11),wood)
        sphere('Cotton tail',(0,.36,.6),(.16,.13,.16),cream)
    elif index==14:
        cylinder('Wide bear barrel',(0,0,.61),.44,.90,body,False,.16)
        sphere('Honey belly',(0,-.42,.6),(.30,.025,.31),mark)
        head((0,-.02,1.22),(.39,.32,.31),ears='round');arms(.79,.45)
    elif index==15:
        cube('Tall cat block',(0,0,.73),(.54,.45,1.19),body,.15)
        head((0,0,1.46),(.34,.26,.30),ears='point');arms(.8,.28)
        tail([(.22,.16,.33),(.53,.16,.45),(.56,.16,.95),(.40,.15,1.05)],.065);dots(.75,-.235)
    elif index==16:
        sphere('Low hedgehog dome',(0,.06,.48),(.53,.36,.35),body)
        head((-.29,-.12,.60),(.26,.24,.21))
        for row in range(3):
            for i in range(7):
                a=(i-3)*.35
                cone('Wooden spine',(math.sin(a)*.44,.10+row*.09,.52+math.cos(a)*.28),.065,.19,endwood,(0,a,0))
    elif index==17:
        sphere('Tiny mouse teardrop',(0,.04,.40),(.31,.26,.27),body)
        head((0,-.06,.79),(.25,.22,.23),ears='round');arms(.47,.25)
        tail([(0,.24,.28),(.40,.28,.14),(.61,.20,.20),(.57,.10,.36)],.026)
    elif index==18:
        sphere('Squat frog body',(0,0,.52),(.44,.30,.29),body)
        for side in [-1,1]:
            st=set(bpy.data.objects)
            sphere('Big folded hind leg',(side*.40,0,.26),(.24,.25,.23),body)
            sphere('Broad frog foot',(side*.40,-.14,.08),(.24,.28,.075),mark)
            group('leg'+str(side),st,(side*.30,0,.35))
        head((0,-.015,.82),(.44,.27,.22));arms(.45,.29)
        for side in [-1,1]:sphere('Raised frog eye',(side*.27,-.07,1.03),(.14,.18,.13),body)
    elif index==20:
        sphere('Squirrel pear',(0,0,.60),(.31,.28,.43),body)
        st=set(bpy.data.objects)
        curve_beam('Great curled squirrel tail',[(.20,.17,.30),(.56,.20,.57),(.58,.20,1.12),(.38,.20,1.36),(.21,.20,1.15)],.17,mark)
        group('tail',st,(.20,.17,.30));head((-.06,-.04,1.15),(.28,.26,.28),ears='point');arms(.75,.27)
        st=set(bpy.data.objects);sphere('Acorn',(0,-.34,.62),(.15,.13,.18),endwood);sphere('Acorn cap',(0,-.34,.74),(.17,.14,.07),ochre);group('prop',st,(0,-.34,.64))
    elif index==21:
        cube('Raccoon barrel block',(0,0,.63),(.72,.50,.87),body,.18)
        head((0,0,1.21),(.38,.28,.29),ears='round',patch=True);arms(.7,.37)
        st=set(bpy.data.objects)
        for i in range(6):sphere('Ringed tail',(.25+i*.07,.19,.26+i*.045),(.13,.13,.10),black if i%2 else body)
        group('tail',st,(.25,.19,.26))
    elif index==22:
        sphere('Panda round belly',(0,0,.57),(.44,.32,.43),cream)
        head((0,0,1.23),(.41,.31,.36),ears='round',patch=True);arms(.76,.43)
        st=set(bpy.data.objects);beam('Bamboo',(0,-.37,.30),(.13,-.37,1.03),.06,.06,leaf)
        sphere('Bamboo leaf',(.17,-.37,.87),(.13,.018,.04),leaf);group('prop',st,(.06,-.37,.67))
    else:
        sphere('Long otter body',(0,.04,.70),(.26,.25,.57),body)
        head((0,-.03,1.34),(.29,.25,.25),ears='round');arms(.85,.27)
        tail([(0,.21,.33),(.27,.26,.13),(.47,.22,.15)],.085)
        st=set(bpy.data.objects);sphere('Little fish',(0,-.34,.66),(.23,.07,.10),teal)
        extrude('Fish tail',[(.17,.66),(.33,.79),(.33,.54)],-.4,-.30,ochre,.015);group('prop',st,(0,-.34,.66))
    if index not in (12,13,18,19):
        for side in [-1,1]:
            st=set(bpy.data.objects);foot(side*(.29 if index in (2,7,14,22) else .18),size=.17 if index==14 else .13)
            group('foot'+str(side),st,(side*.18,0,.16))
    parts=set(bpy.data.objects)-before
    top=[o for o in parts if o.parent is None]
    root=joint('Whole solid toy',top,(0,0,0));rig['root']=root
    rest={k:(v.location.copy(),v.rotation_euler.copy(),v.scale.copy())for k,v in rig.items()}
    def pose(phase):
        for k,o in rig.items():o.location,o.rotation_euler,o.scale=rest[k]
        w=math.sin(phase);q=1-math.cos(phase)
        def rotate(key,axis,value):
            if key in rig:rig[key].rotation_euler[axis]+=value
        def move(key,axis,value):
            if key in rig:rig[key].location[axis]+=value
        if index==0:rotate('head',0,q*.16);rotate('wing-1',1,-w*.22);rotate('wing1',1,w*.22)
        elif index==1:move('root',2,abs(w)*.18);rotate('head',1,w*.12)
        elif index==2:rotate('head',2,w*.48);rotate('wing1',1,q*.13)
        elif index==3:rotate('wing-1',1,-w*.65);rotate('wing1',1,w*.65);move('root',2,q*.035)
        elif index==4:rotate('crest',0,-q*.24);rotate('head',1,w*.17)
        elif index==5:rotate('root',1,w*.12);move('foot-1',2,max(0,w)*.12);move('foot1',2,max(0,-w)*.12)
        elif index==6:rotate('head',2,w*.30);rotate('head',0,q*.14)
        elif index==7:rotate('tail',2,w*.45);rotate('root',1,w*.10);move('foot1',2,max(w,0)*.10)
        elif index==8:move('root',2,abs(math.sin(phase*2))*.10);rotate('tail',2,w*.5)
        elif index==9:rotate('head',0,q*.28);rotate('crest',0,q*.28);rotate('root',0,q*.07)
        elif index==10:rotate('head',0,q*.16);rotate('wing-1',1,-q*.13);rotate('wing1',1,q*.13)
        elif index==11:rotate('fan',2,w*.14);rotate('fan',0,q*.13);rotate('head',2,-w*.16)
        elif index==12:rotate('tail',2,w*.55);rotate('head',1,w*.20)
        elif index==13:move('root',2,abs(w)*.25);rotate('ear-1',1,w*.18);rotate('ear1',1,-w*.18)
        elif index==14:rotate('arm1',1,-q*.55);rotate('head',1,w*.09)
        elif index==15:rotate('tail',1,w*.28);rotate('head',1,-w*.28)
        elif index==16:rotate('head',2,w*.26);move('root',0,w*.045)
        elif index==17:rotate('arm-1',0,math.sin(phase*3)*.48);rotate('arm1',0,-math.sin(phase*3)*.48);rotate('head',0,q*.10)
        elif index==18:move('root',2,q*.15);rotate('leg-1',1,-q*.13);rotate('leg1',1,q*.13)
        elif index==19:rotate('leg-1-1',0,q*.35);rotate('head',2,w*.20);rotate('antlers',2,w*.20)
        elif index==20:move('prop',2,q*.07);rotate('head',0,q*.18);rotate('tail',1,w*.13)
        elif index==21:rotate('head',2,w*.44);rotate('tail',2,-w*.30);rotate('arm-1',0,q*.17)
        elif index==22:rotate('prop',1,w*.17);rotate('head',0,q*.13);rotate('arm1',0,q*.15)
        else:rotate('prop',1,w*.28);rotate('root',1,w*.10);rotate('arm-1',0,w*.17);rotate('arm1',0,-w*.17)
    # Twelve articulated poses and thirteen actual 3D turn views (0..180deg).
    # A single atlas per visitor keeps playback native and fully opaque.
    from array import array
    tile=192;cols=5;rows=5
    pixels=array('f',[0.])*(tile*cols*tile*rows*4)
    for frame in range(25):
        pose(frame*math.tau/12 if frame<12 else 0)
        if frame>=12:root.rotation_euler[2]=(frame-12)*math.pi/12
        path=ROOT/'artifacts'/('visitor-frame-%02d.png'%index)
        bpy.context.scene.render.filepath=str(path);bpy.ops.render.render(write_still=True)
        im=bpy.data.images.load(str(path),check_existing=False)
        data=array('f',[0.])*(tile*tile*4);im.pixels.foreach_get(data)
        col=frame%cols;row=rows-1-frame//cols
        for y in range(tile):
            dest=((row*tile+y)*tile*cols+col*tile)*4
            pixels[dest:dest+tile*4]=data[y*tile*4:(y+1)*tile*4]
        if frame==0:
            im.filepath_raw=str(OUT/('visitor-%02d.png'%index));im.file_format='PNG';im.save()
        bpy.data.images.remove(im)
    atlas=bpy.data.images.new('Visitor atlas',width=tile*cols,height=tile*rows,alpha=True)
    atlas.pixels.foreach_set(pixels);atlas.filepath_raw=str(OUT/('visitor-atlas-%02d.png'%index));atlas.file_format='PNG';atlas.save()
    bpy.data.images.remove(atlas)
    print('CHARACTER COMPLETE',index,flush=True)

if ONLY in ('all','house'):clock_body()
if ONLY in ('all','pendulum'):pendulum()
if ONLY in ('all','doors'):doors()
if ONLY in ('all','characters') or ONLY.isdigit():
    source=(ROOT/'src'/'cast.rs').read_text()
    colors=re.findall(r'color: 0x([0-9a-f]+),\s*accent: 0x([0-9a-f]+)',source)
    for i,(c,a) in enumerate(colors):
        if ONLY.isdigit() and i!=int(ONLY):continue
        character(i,int(c,16),int(a,16))
print('RENDERING COMPLETE',flush=True)
