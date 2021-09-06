use core::convert::{TryFrom, TryInto};

fn add<A, T>(args: A) -> Result<T, <T as TryInto<isize>>::Error>
where A: IntoIterator<Item=T>,
      T: From<isize> + TryInto<isize>,
{
    args.into_iter()
        .try_fold(0, |sum, arg| arg.try_into().map(|x| sum + x))
        .map(T::from)
}

fn sub<A, T>(args: A) -> Result<T, <T as TryInto<isize>>::Error>
where A: IntoIterator<Item=T>,
      isize: TryFrom<T>,
      T: From<isize>,
      <A as IntoIterator>::IntoIter: ExactSizeIterator,
{
    let i = args.into_iter();
    if let Some(x) = i.next().map(isize::try_from) {
        if let Some(y) = i.next().map(isize::try_from) {
            i.try_fold(x? - y?, |sum, arg| isize::try_from(arg).map(|x| sum - x))
                .map(T::from)
        } else {
            x.map(|x| T::from(0 - x))
        }
    } else {
        Ok(T::from(0))
    }
}

fn mul<A, T>(args: A) -> Result<isize, <T as TryInto<isize>>::Error>
where A: IntoIterator<Item=T>,
      T: TryInto<isize> {
    args.into_iter().try_fold(1, |acc, arg| Ok(acc * arg.try_into()?))
}

fn div<A, T>(args: A) -> Result<T, <T as TryInto<isize>>::Error>
where A: IntoIterator<Item=T>,
      isize: TryFrom<T>,
      T: From<isize>,
      <A as IntoIterator>::IntoIter: ExactSizeIterator,
{
    let i = args.into_iter();
    if let Some(x) = i.next().map(isize::try_from) {
        if let Some(y) = i.next().map(isize::try_from) {
            i.try_fold(x? / y?, |sum, arg| isize::try_from(arg).map(|x| sum / x))
                .map(T::from)
        } else {
            x.map(|x| T::from(x))
        }
    } else {
        Ok(T::from(0))
    }
}


// lambda
// def
/*

#[special]
pub fn lambda(...) {
}

#[special]
pub fn def(...) {
}

#[special]
pub fn quote(...) {
}

#[special]
pub fn quasiquote(...) {
}

#[special]
pub fn if_(...) {
}

#[special]
pub fn and(...) {
}

#[special]
pub fn or(...) {
}

#[special]
pub fn do(...) {
}


#[function]
pub fn add(...) {
}

*/
