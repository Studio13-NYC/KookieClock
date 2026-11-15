#pragma once

#import <AppKit/AppKit.h>

#include "ClockCore.h"

@interface ClockView : NSView
@property(nonatomic) KCFrame frameState;
@property(nonatomic, copy) NSDictionary *settings;
- (NSImage *)art:(NSString *)name;
@end
