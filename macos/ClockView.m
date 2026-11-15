#import "ClockView.h"
#import <ImageIO/ImageIO.h>

static NSColor *Paint(unsigned rgb, CGFloat alpha) {
    return [NSColor colorWithSRGBRed:((rgb >> 16) & 255) / 255.0
                             green:((rgb >> 8) & 255) / 255.0
                              blue:(rgb & 255) / 255.0 alpha:alpha];
}

static void FillOval(NSRect rect, unsigned rgb, CGFloat alpha) {
    [Paint(rgb, alpha) setFill];
    [[NSBezierPath bezierPathWithOvalInRect:rect] fill];
}

static void DrawImage(NSImage *image, NSRect rect) {
    [image drawInRect:rect fromRect:NSZeroRect operation:NSCompositingOperationSourceOver
             fraction:1 respectFlipped:YES hints:nil];
}

static NSPoint RotateHand(CGFloat x, CGFloat y, CGFloat a) {
    return NSMakePoint(200 + x * cos(a) - y * sin(a), 327 + x * sin(a) + y * cos(a));
}

static void DrawHand(CGFloat angle, CGFloat length, CGFloat width) {
    const NSPoint points[] = {
        {-width * .45, 10}, {-width, 0}, {-width * .60, -length * .76},
        {0, -length}, {width * .60, -length * .76}, {width, 0}, {width * .45, 10},
    };
    NSBezierPath *shape = [NSBezierPath bezierPath];
    NSBezierPath *shadow = [NSBezierPath bezierPath];
    for (NSUInteger i = 0; i < sizeof(points) / sizeof(points[0]); i++) {
        NSPoint p = RotateHand(points[i].x, points[i].y, angle);
        NSPoint shade = NSMakePoint(p.x + 1.5, p.y + 2);
        if (i == 0) {
            [shape moveToPoint:p];
            [shadow moveToPoint:shade];
        } else {
            [shape lineToPoint:p];
            [shadow lineToPoint:shade];
        }
    }
    [shape closePath];
    [shadow closePath];
    [Paint(0x0d1819, .21) setFill];
    [shadow fill];
    [Paint(0x344b49, 1) setFill];
    [shape fill];
    [Paint(0x718783, 1) setStroke];
    NSBezierPath *shine = [NSBezierPath bezierPath];
    shine.lineWidth = .65;
    [shine moveToPoint:RotateHand(-width * .35, 0, angle)];
    [shine lineToPoint:RotateHand(-width * .24, -length * .71, angle)];
    [shine stroke];
}

@implementation ClockView {
    NSCache<NSString *, NSImage *> *_images;
    NSCache<NSNumber *, NSImage *> *_poses;
    NSDictionary<NSString *, NSImage *> *_visitArt;
}

- (instancetype)initWithFrame:(NSRect)frame {
    if ((self = [super initWithFrame:frame])) {
        _images = [NSCache new];
        _images.countLimit = 32;
        _poses = [NSCache new];
        _poses.countLimit = 30;
        _frameState.visitor = -1;
        self.accessibilityLabel = @"KookieClock. Right-click for controls.";
    }
    return self;
}

- (BOOL)isFlipped { return YES; }
- (BOOL)acceptsFirstMouse:(NSEvent *)event { return YES; }
- (void)mouseDown:(NSEvent *)event {
    if (![self.settings[@"locked"] boolValue]) {
        [self.window performWindowDragWithEvent:event];
    }
}

- (NSImage *)art:(NSString *)name {
    NSImage *image = _visitArt[name] ?: [_images objectForKey:name];
    if (image) return image;
    NSURL *url = [NSBundle.mainBundle URLForResource:name withExtension:@"png"
                                       subdirectory:@"woodland"];
    if (!url) return nil;
    CGImageSourceRef source = CGImageSourceCreateWithURL((__bridge CFURLRef)url, NULL);
    if (!source) return nil;
    CGImageRef bitmap = CGImageSourceCreateImageAtIndex(source, 0, (__bridge CFDictionaryRef)@{
        (__bridge NSString *)kCGImageSourceShouldCacheImmediately: @YES
    });
    CFRelease(source);
    if (!bitmap) return nil;
    image = [[NSImage alloc] initWithCGImage:bitmap
        size:NSMakeSize(CGImageGetWidth(bitmap), CGImageGetHeight(bitmap))];
    CGImageRelease(bitmap);
    if (image) [_images setObject:image forKey:name];
    return image;
}

- (void)prepareVisit:(int)character {
    // Keep these images alive throughout the performance, even under memory pressure.
    NSMutableDictionary *art = [NSMutableDictionary dictionaryWithCapacity:18];
    for (int i = 0; i <= 16; ++i) {
        NSString *name = [NSString stringWithFormat:@"doors-%02d", i];
        art[name] = [self art:name];
    }
    NSString *name = [NSString stringWithFormat:@"visitor-atlas-%02d", character];
    art[name] = [self art:name];
    _visitArt = art;
}

- (NSImage *)sprite:(int)visitor pose:(int)pose {
    NSNumber *key = @(visitor * 25 + pose);
    NSImage *cached = [_poses objectForKey:key];
    if (cached) return cached;
    NSImage *atlas = [self art:[NSString stringWithFormat:@"visitor-atlas-%02d", visitor]];
    CGImageRef source = [atlas CGImageForProposedRect:NULL context:nil hints:nil];
    if (!source) return nil;
    CGRect rect = CGRectMake((pose % 5) * 192, (pose / 5) * 192, 192, 192);
    CGImageRef tile = CGImageCreateWithImageInRect(source, rect);
    if (!tile) return nil;
    NSImage *image = [[NSImage alloc] initWithCGImage:tile size:NSMakeSize(192, 192)];
    CGImageRelease(tile);
    [_poses setObject:image forKey:key];
    return image;
}

- (void)drawRect:(NSRect)dirty {
    KCFrame state = self.frameState;
    [NSGraphicsContext saveGraphicsState];
    [[NSColor clearColor] setFill];
    NSRectFillUsingOperation(dirty, NSCompositingOperationCopy);
    NSGraphicsContext.currentContext.imageInterpolation = NSImageInterpolationHigh;
    NSAffineTransform *scale = [NSAffineTransform transform];
    [scale scaleXBy:self.bounds.size.width / 400 yBy:self.bounds.size.height / 575];
    [scale concat];

    [NSGraphicsContext saveGraphicsState];
    NSAffineTransform *pendulum = [NSAffineTransform transform];
    [pendulum translateXBy:200 yBy:365];
    [pendulum rotateByDegrees:state.swing];
    [pendulum concat];
    DrawImage([self art:@"pendulum"], NSMakeRect(-64, -16, 128, 256));
    [NSGraphicsContext restoreGraphicsState];
    DrawImage([self art:@"house"], NSMakeRect(0, 0, 400, 575));

    if (state.visitor >= 0) {
        [NSGraphicsContext saveGraphicsState];
        NSBezierPath *doorway = [NSBezierPath bezierPath];
        [doorway moveToPoint:NSMakePoint(143, 225)];
        [doorway lineToPoint:NSMakePoint(143, 161)];
        [doorway appendBezierPathWithArcWithCenter:NSMakePoint(200, 161)
                                           radius:57 startAngle:180 endAngle:360 clockwise:NO];
        [doorway lineToPoint:NSMakePoint(257, 225)];
        [doorway closePath];
        [doorway addClip];
        CGFloat proportion = 1.05;
        switch (state.visitor) {
            case 13: proportion = .86; break;
            case 4: case 9: case 19: proportion = .91; break;
            case 15: proportion = .96; break;
        }
        CGFloat size = proportion * (.82 + .18 * state.depth);
        CGFloat x = 200 + 130 * state.lateral;
        CGFloat feet = 204 + 18 * state.depth;
        CGFloat stride = sin(state.depth * M_PI);
        CGFloat step = sin(state.elapsed * 13);
        FillOval(NSMakeRect(x - 25 * size, feet - 3, 50 * size, 6), 0x081112, .16);
        NSAffineTransform *visitor = [NSAffineTransform transform];
        [visitor translateXBy:x yBy:feet - fabs(step) * stride * 2];
        [visitor rotateByDegrees:step * stride * 2];
        [visitor scaleBy:size];
        [visitor concat];
        DrawImage([self sprite:state.visitor pose:state.sprite], NSMakeRect(-80, -137.143, 160, 160));
        [NSGraphicsContext restoreGraphicsState];
    }
    DrawImage([self art:[NSString stringWithFormat:@"doors-%02d", state.door]],
              NSMakeRect(90, 90, 220, 145));

    CGFloat minutes = state.minute + state.second / 60.0;
    DrawHand((state.hour % 12 + minutes / 60) * M_PI / 6, 47, 3.9);
    DrawHand(minutes * M_PI / 30, 69, 2.5);
    if ([self.settings[@"seconds"] boolValue]) {
        CGFloat angle = state.second * M_PI / 30;
        NSBezierPath *second = [NSBezierPath bezierPath];
        [second moveToPoint:RotateHand(0, 12, angle)];
        [second lineToPoint:RotateHand(0, -78, angle)];
        second.lineWidth = 1;
        [Paint(0xae694a, 1) setStroke];
        [second stroke];
    }
    FillOval(NSMakeRect(195, 323, 13, 13), 0x0d1819, .2);
    FillOval(NSMakeRect(194.5, 321.5, 11, 11), 0xb18b53, 1);
    FillOval(NSMakeRect(195.5, 322, 8.5, 8.5), 0xdcc291, 1);
    FillOval(NSMakeRect(198, 324, 3.5, 3.5), 0xf0dfb8, 1);
    if ([self.settings[@"digital"] boolValue]) {
        int hour = [self.settings[@"clock_24h"] boolValue] ? state.hour : (state.hour + 11) % 12 + 1;
        NSString *time = [NSString stringWithFormat:@"%d:%02d", hour, state.minute];
        NSDictionary *style = @{
            NSFontAttributeName: [NSFont monospacedDigitSystemFontOfSize:11 weight:NSFontWeightMedium],
            NSForegroundColorAttributeName: Paint(0x58645c, 1),
        };
        [time drawAtPoint:NSMakePoint(200 - [time sizeWithAttributes:style].width / 2, 365)
           withAttributes:style];
    }
    [NSGraphicsContext restoreGraphicsState];
}
@end
