/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! The `UIToolbar` class stub.

use super::ui_view::UIViewHostObject;
use crate::objc::{id, msg_class, objc_classes, ClassExports, NSZonePtr};

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    @implementation UIToolbar:UIView

    + (id)allocWithZone:(NSZonePtr)_zone {
        msg_class![env; _touchHLE_UIToolbar_Internal allocWithZone:_zone]
    }

    @end

    @implementation _touchHLE_UIToolbar_Internal:UIView

    + (id)allocWithZone:(NSZonePtr)_zone {
        // We use UIViewHostObject because UIToolbar inherits from UIView
        // and UIView methods expect to find this specific type of host object.
        env.objc.alloc_object(this, Box::<UIViewHostObject>::default(), &mut env.mem)
    }

    - (id)init {
        this
    }

    - (())setItems:(id)_items {
        // Stub: ignore the installation of elements
    }

    - (())setItems:(id)_items animated:(bool)_animated {
        // Stub: ignore the installation of elements with animation
    }

    @end
};
