/* THE ORIGINAL VERSION OF THIS FILE WAS DISTRIBUTED WITH THE FOLLOWING LICENSE

"""
MIT License
Copyright (c) 2017 Rust for Robotics Developers
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

THE CURRENT FORM OF THIS FILE IS LICENSED UNDER THE SAME TERMS AS THE REST OF THIS REPOSITORY.
SEE THE LICENSE FILE FOR FULL TERMS.
*/

use super::hal::tInstances;
use super::hal::tResourceType;
use super::hal::HAL_Report;
use std::os::raw;
use std::ptr;

/// Report the usage of a specific resource type with an `instance` value attached.
pub fn report_usage(resource: tResourceType, instance: tInstances) {
    unsafe {
        HAL_Report(resource as i32, instance as i32, 0, ptr::null());
    }
}

#[allow(dead_code)]
/// A safe wrapper around HAL_Report
pub fn report_usage_extras(
    resource: tResourceType,
    instance: tInstances,
    context: i32,
    feature: *const raw::c_char,
) {
    unsafe {
        HAL_Report(resource as i32, instance as i32, context, feature);
    }
}