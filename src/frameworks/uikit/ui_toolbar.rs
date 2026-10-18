/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! The `UIToolbar` class stub.

use crate::objc::{id, msg_class, objc_classes, ClassExports, HostObject, NSZonePtr};

pub struct UIToolbarHostObject;
impl HostObject for UIToolbarHostObject {}

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    @implementation UIToolbar:UIView

    + (id)allocWithZone:(NSZonePtr)_zone {
        msg_class![env; _touchHLE_UIToolbar_Internal allocWithZone:_zone]
    }

    @end

    @implementation _touchHLE_UIToolbar_Internal:UIView

    + (id)allocWithZone:(NSZonePtr)_zone {
        env.objc.alloc_object(this, Box::new(UIToolbarHostObject), &mut env.mem)
    }

    - (id)init {
        this
    }

    @end
};
