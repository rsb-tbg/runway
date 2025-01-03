extern crate opencv as cv;
extern crate mongodb;
extern crate tokio;
extern crate serde;

use std::{fmt, io, error::{self}};

use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug)]
pub struct RunwayError {
    pub msg: String,
}

impl Serialize for RunwayError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("RunwayError", 1)?;
        s.serialize_field("msg", &self.msg)?;
        s.end()
    }
}

impl error::Error for RunwayError {}

impl fmt::Display for RunwayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}", &self.msg)
    }
}

impl From<io::Error> for RunwayError {
    fn from(value: io::Error) -> Self {
        Self { msg: value.to_string() }
    }
}

impl From<cv::Error> for RunwayError {
    fn from(value: cv::Error) -> Self {
        Self { msg: "OpenCV error: ".to_owned() + &value.message }
    }
}

impl From<mongodb::error::Error> for RunwayError {
    fn from(value: mongodb::error::Error) -> Self {
        Self { msg: "mongoDB error: ".to_owned() + &value.kind.to_string() }
    }

}