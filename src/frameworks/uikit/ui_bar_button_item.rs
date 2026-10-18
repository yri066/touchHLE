/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `UIBarButtonItem` class.

use crate::objc::{id, objc_classes, ClassExports, NSZonePtr};
// Use super::ui_view::UIView; if UIBarItem not exist
use super::ui_responder::UIResponder;

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    // UIBarItem is absent; UIResponder is used.
    @implementation UIBarButtonItem:UIResponder

    + (id)allocWithZone:(NSZonePtr)_zone {
        let host_object = Box::new(UIBarButtonItemHostObject::default());
        env.objc.alloc_object(this, host_object, &mut env.mem)
    }

    // Stub for initWithImage:style:target:action:
    - (id)initWithImage:(id)image
                style:(i32)style
             target:(id)target
             action:(SEL)action {
        log_dbg!("[UIBarButtonItem initWithImage:{:?} style:{} target:{:?} action:{}]", image, style, target, action.as_str(&env.mem));
        this
    }

    // Stub for initWithTitle:style:target:action:
    - (id)initWithTitle:(id)title
                style:(i32)style
             target:(id)target
             action:(SEL)action {
        log_dbg!("[UIBarButtonItem initWithTitle:{:?} style:{} target:{:?} action:{}]", title, style, target, action.as_str(&env.mem));
        this
    }

    @end
};

#[derive(Default)]
struct UIBarButtonItemHostObject {
    // Here you can add fields for
    // storing the button state in the future
}

impl crate::objc::HostObject for UIBarButtonItemHostObject {}
