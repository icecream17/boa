use std::ops::Deref;

use crate::{
    builtins::Array,
    gc::{Finalize, Trace},
    object::JsObject,
    Context, JsResult, JsString, JsValue,
};

// TODO: put in a better place.
pub trait JsObjectType: Into<JsValue> + Into<JsObject> + Deref<Target = JsObject> {}

/// JavaScript `Array` rust object.
#[derive(Debug, Clone, Trace, Finalize)]
pub struct JsArray {
    inner: JsObject,
}

impl JsArray {
    #[inline]
    pub fn new(array: &[JsValue], context: &mut Context) -> Self {
        let inner = Array::create_array_from_list(array.to_vec(), context);

        Self { inner }
    }

    #[inline]
    pub fn from(object: JsObject, context: &mut Context) -> JsResult<Self> {
        if object.borrow().is_array() {
            Ok(Self { inner: object })
        } else {
            context.throw_type_error("object is not an Array")
        }
    }

    #[inline]
    pub fn length(&self, context: &mut Context) -> JsResult<usize> {
        self.inner.length_of_array_like(context)
    }

    #[inline]
    pub fn is_empty(&self, context: &mut Context) -> JsResult<bool> {
        self.inner.length_of_array_like(context).map(|len| len == 0)
    }

    #[inline]
    pub fn push<T>(&self, value: T, context: &mut Context) -> JsResult<JsValue>
    where
        T: Into<JsValue>,
    {
        self.push_items(&[value.into()], context)
    }

    #[inline]
    pub fn push_items(&self, items: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Array::push(&self.inner.clone().into(), items, context)
    }

    #[inline]
    pub fn pop(&self, context: &mut Context) -> JsResult<JsValue> {
        Array::pop(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn at<T>(&self, index: T, context: &mut Context) -> JsResult<JsValue>
    where
        T: Into<i64>,
    {
        Array::at(&self.inner.clone().into(), &[index.into().into()], context)
    }

    #[inline]
    pub fn shift(&self, context: &mut Context) -> JsResult<JsValue> {
        Array::shift(&self.inner.clone().into(), &[], context)
    }

    #[inline]
    pub fn unshift(&self, items: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Array::shift(&self.inner.clone().into(), items, context)
    }

    #[inline]
    pub fn reverse(&self, context: &mut Context) -> JsResult<Self> {
        Array::reverse(&self.inner.clone().into(), &[], context)?;
        Ok(self.clone())
    }

    #[inline]
    pub fn concat(&self, items: &[JsValue], context: &mut Context) -> JsResult<Self> {
        let object = Array::concat(&self.inner.clone().into(), items, context)?
            .as_object()
            .cloned()
            .expect("Array.prototype.filter should always return object");

        Self::from(object, context)
    }

    #[inline]
    pub fn join(&self, separator: Option<JsString>, context: &mut Context) -> JsResult<JsString> {
        Array::join(&self.inner.clone().into(), &[separator.into()], context).map(|x| {
            x.as_string()
                .cloned()
                .expect("Array.prototype.join always returns string")
        })
    }

    #[inline]
    pub fn fill<T>(
        &self,
        value: T,
        start: Option<u32>,
        end: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Self>
    where
        T: Into<JsValue>,
    {
        Array::fill(
            &self.inner.clone().into(),
            &[value.into(), start.into(), end.into()],
            context,
        )?;
        Ok(self.clone())
    }

    #[inline]
    pub fn index_of<T>(
        &self,
        search_element: T,
        from_index: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Option<u32>>
    where
        T: Into<JsValue>,
    {
        let index = Array::index_of(
            &self.inner.clone().into(),
            &[search_element.into(), from_index.into()],
            context,
        )?
        .as_number()
        .expect("Array.prototype.indexOf should always return number");

        if index == -1.0 {
            Ok(None)
        } else {
            Ok(Some(index as u32))
        }
    }

    #[inline]
    pub fn last_index_of<T>(
        &self,
        search_element: T,
        from_index: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Option<u32>>
    where
        T: Into<JsValue>,
    {
        let index = Array::last_index_of(
            &self.inner.clone().into(),
            &[search_element.into(), from_index.into()],
            context,
        )?
        .as_number()
        .expect("Array.prototype.lastIndexOf should always return number");

        if index == -1.0 {
            Ok(None)
        } else {
            Ok(Some(index as u32))
        }
    }

    #[inline]
    pub fn find(
        &self,
        predicate: JsObject,
        this_arg: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Array::find(
            &self.inner.clone().into(),
            &[predicate.into(), this_arg.into()],
            context,
        )
    }

    #[inline]
    pub fn filter(
        &self,
        callback: JsObject,
        this_arg: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<Self> {
        let object = Array::filter(
            &self.inner.clone().into(),
            &[callback.into(), this_arg.into()],
            context,
        )?
        .as_object()
        .cloned()
        .expect("Array.prototype.filter should always return object");

        Self::from(object, context)
    }

    #[inline]
    pub fn map(
        &self,
        callback: JsObject,
        this_arg: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<Self> {
        let object = Array::map(
            &self.inner.clone().into(),
            &[callback.into(), this_arg.into()],
            context,
        )?
        .as_object()
        .cloned()
        .expect("Array.prototype.map should always return object");

        Self::from(object, context)
    }

    #[inline]
    pub fn every(
        &self,
        callback: JsObject,
        this_arg: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<bool> {
        let result = Array::every(
            &self.inner.clone().into(),
            &[callback.into(), this_arg.into()],
            context,
        )?
        .as_boolean()
        .expect("Array.prototype.every should always return boolean");

        Ok(result)
    }

    #[inline]
    pub fn some(
        &self,
        callback: JsObject,
        this_arg: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<bool> {
        let result = Array::some(
            &self.inner.clone().into(),
            &[callback.into(), this_arg.into()],
            context,
        )?
        .as_boolean()
        .expect("Array.prototype.some should always return boolean");

        Ok(result)
    }

    #[inline]
    pub fn sort(&self, compare_fn: Option<JsObject>, context: &mut Context) -> JsResult<Self> {
        Array::sort(&self.inner.clone().into(), &[compare_fn.into()], context)?;

        Ok(self.clone())
    }

    #[inline]
    pub fn slice(
        &self,
        start: Option<u32>,
        end: Option<u32>,
        context: &mut Context,
    ) -> JsResult<Self> {
        let object = Array::slice(
            &self.inner.clone().into(),
            &[start.into(), end.into()],
            context,
        )?
        .as_object()
        .cloned()
        .expect("Array.prototype.slice should always return object");

        Self::from(object, context)
    }

    #[inline]
    pub fn reduce(
        &self,
        callback: JsObject,
        initial_value: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Array::reduce(
            &self.inner.clone().into(),
            &[callback.into(), initial_value.into()],
            context,
        )
    }

    #[inline]
    pub fn reduce_right(
        &self,
        callback: JsObject,
        initial_value: Option<JsValue>,
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Array::reduce_right(
            &self.inner.clone().into(),
            &[callback.into(), initial_value.into()],
            context,
        )
    }

    // TODO: Other Array methods
}

impl From<JsArray> for JsObject {
    #[inline]
    fn from(o: JsArray) -> Self {
        o.inner.clone()
    }
}

impl From<JsArray> for JsValue {
    #[inline]
    fn from(o: JsArray) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsArray {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsArray {}
