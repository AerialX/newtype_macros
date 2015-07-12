#[allow(unused_imports)]
use std::convert::{From,Into};
#[allow(unused_imports)]
use std::ops::{Add,Mul,Sub,Div,Neg,Deref};
#[allow(unused_imports)]
use std::fmt::{self,Display};

#[macro_export]
macro_rules! newtype_derive {
	() => (());
	($alias:ident, $t:ty, Deref) => {
		impl Deref for $alias {
			type Target = $t;
			fn deref<'a>(&'a self) -> &'a $t {
				let $alias(ref v) = *self;
				v
			}
		}		
	};
	($alias:ident, $t:ty, From) => {
		impl From<$t> for $alias {
			fn from(v: $t) -> Self {
				$alias(v)
			}
		}
	};
	($alias:ident, $t:ty, Into) => {
		impl Into<$t> for $alias {
			fn into(self) -> $t {
				let $alias(v) = self;
				v
			}
		}
	};
	($alias:ident, $t:ty, Display) => {
		impl Display for $alias {
			 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 			let $alias(ref v) = *self;
	 			v.fmt(f)
	 		}
		}
	};
	($alias:ident, $t:ty, Add) => {
		impl Add for $alias {
			type Output = $alias;
			fn add(self, _rhs: $alias) -> Self {
				let $alias(ref l) = self;
				let $alias(ref r) = _rhs;
			 	$alias::from(l + r)
	 		}
		}
	};
	($alias:ident, $t:ty, Sub) => {
		impl Sub for $alias {
			type Output = $alias;
			fn sub(self, _rhs: $alias) -> Self {
				let $alias(ref l) = self;
				let $alias(ref r) = _rhs;
			 	$alias::from(l - r)
	 		}
		}
	};
	($alias:ident, $t:ty, Mul) => {
		impl Mul for $alias {
			type Output = $alias;
			fn mul(self, _rhs: $alias) -> Self {
				let $alias(ref l) = self;
				let $alias(ref r) = _rhs;
			 	$alias::from(l * r)
	 		}
		}
	};
	($alias:ident, $t:ty, Div) => {
		impl Div for $alias {
			type Output = $alias;
			fn div(self, _rhs: $alias) -> Self {
				let $alias(ref l) = self;
				let $alias(ref r) = _rhs;
			 	$alias::from(l / r)
	 		}
		}
	};
	($alias:ident, $t:ty, Neg) => {
		impl Neg for $alias {
			type Output = $alias;
			fn neg(self) -> Self {
				let $alias(ref v) = self;
			 	$alias::from(-v)
	 		}
		}
	};
	($alias:ident, $t:ty, $($keyword:ident),*) => {
		$(newtype_derive!($alias, $t, $keyword);)*
	};

}

macro_rules! newtype {
	($alias:ident, $t:ty, $($keyword:ident),*) => {
		#[derive(Debug,PartialEq,PartialOrd)]
		struct $alias($t);
		$(newtype_derive!($alias, $t, $keyword);)*
	}
}

#[test]
fn test_newtype_derive() {
	struct Miles(u32);
	newtype_derive!(Miles,u32,Display,From,Into,Deref);
	let m = Miles::from(14);
	let m2:Miles = 14.into();
	assert_eq!(*m,14);
	assert_eq!(*m2,14);
	assert_eq!(String::from("14"),format!("{}",m));

}

#[test]
fn test_newtype() {
	newtype!(Miles,u32,Display,From,Into,Deref);
	let m = Miles::from(14);
	let m2:Miles = 14.into();
	assert_eq!(*m,14);
	assert_eq!(*m2,14);
	assert_eq!(String::from("14"),format!("{}",m));

}

#[test]
fn test_add() {
	newtype!(Miles,u32,From,Add);
	let m = Miles::from(14);
	let m2 = Miles::from(20);
	assert_eq!(Miles::from(34),m+m2);
}

#[test]
fn test_sub() {
	newtype!(Miles,u32,From,Sub);
	let m = Miles::from(20);
	let m2 = Miles::from(14);
	assert_eq!(Miles::from(6),m-m2);
}

#[test]
fn test_mul() {
	newtype!(Miles,u32,From,Mul);
	let m = Miles::from(14);
	let m2 = Miles::from(20);
	assert_eq!(Miles::from(280),m*m2);
}

#[test]
fn test_div() {
	newtype!(Miles,f64,From,Div);
	let m = Miles::from(20f64);
	let m2 = Miles::from(5f64);
	assert_eq!(Miles::from(4f64),m/m2);
}

#[test]
fn test_neg() {
	newtype!(Miles,i32,From,Neg);
	let m = Miles::from(20);
	assert_eq!(Miles::from(-20),-m);
}
