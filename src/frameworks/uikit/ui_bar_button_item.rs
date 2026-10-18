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
        msg_class![env; _touchHLE_UIBarButtonItem_Internal allocWithZone:_zone]
    }

    @end

    @implementation _touchHLE_UIBarButtonItem_Internal:NSObject

    + (id)allocWithZone:(NSZonePtr)_zone {
        env.objc.alloc_object(this, Box::new(UIBarButtonItemHostObject), &mut env.mem)
    }

    - (id)init {
        this
    }

    - (id)initWithTitle:(id)_title
    style:(i32)_style
    target:(id)_target
    action:(SEL)_action {
        this
    }

    - (id)initWithImage:(id)_image
    style:(i32)_style
    target:(id)_target
    action:(SEL)_action {
        this
    }

    - (id)initWithBarButtonSystemItem:(i32)_systemItem
    target:(id)_target
    action:(SEL)_action {
        this
    }

    - (id)initWithCustomView:(id)_customView {
        this
    }

    @end
};
