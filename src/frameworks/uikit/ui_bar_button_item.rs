/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `UIBarButtonItem` class.

use crate::objc::{id, nil, ClassExports, SEL};

pub const CLASSES: ClassExports = crate::objc::objc_classes! {

(env, this, _cmd);

@implementation UIBarItem : NSObject

+ (id)alloc {
    let host_object = Box::<UIBarItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

@implementation UIBarButtonItem : UIBarItem

+ (id)alloc {
    let host_object = Box::<UIBarButtonItemHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)init {
    this
}

- (id)initWithTitle:(id)title style:(i32)style target:(id)target action:(SEL)action {
    this
}

- (id)initWithImage:(id)image style:(i32)style target:(id)target action:(SEL)action {
    this
}

- (id)initWithBarButtonSystemItem:(i32)systemItem target:(id)target action:(SEL)action {
    this
}

- (id)initWithCustomView:(id)customView {
    this
}

- (())dealloc {
    env.objc.dealloc_object(this, &mut env.mem)
}

@end

};

#[derive(Default)]
struct UIBarItemHostObject {
    // TODO: add fields here if the application
    // attempts to read/write them
}

#[derive(Default)]
struct UIBarButtonItemHostObject {
    // TODO: add fields here if the application
    // attempts to read/write them
}
