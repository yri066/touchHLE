/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `UIBarButtonItem` class.

use crate::objc::{id, msg_class, objc_classes, ClassExports, HostObject, NSZonePtr, SEL};

pub struct UIBarButtonItemHostObject;
impl HostObject for UIBarButtonItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    @implementation UIBarButtonItem:NSObject

    + (id)allocWithZone:(NSZonePtr)_zone {
        let class = env.objc.get_known_class("_touchHLE_UIBarButtonItem_Internal", &mut env.mem);
        msg_class![env; class allocWithZone:_zone]
    }

    @end

    @implementation _touchHLE_UIBarButtonItem_Internal:NSObject

    + (id)allocWithZone:(NSZonePtr)_zone {
        env.objc.alloc_object(this, Box::new(UIBarButtonItemHostObject), &mut env.mem)
    }

    - (id)init {
        this
    }

    - (id)initWithTitle:(id)title
    style:(i32)style
    target:(id)target
    action:(SEL)action {
        this
    }

    - (id)initWithImage:(id)image
    style:(i32)style
    target:(id)target
    action:(SEL)action {
        this
    }

    - (id)initWithBarButtonSystemItem:(i32)systemItem
    target:(id)target
    action:(SEL)action {
        this
    }

    @end
};
