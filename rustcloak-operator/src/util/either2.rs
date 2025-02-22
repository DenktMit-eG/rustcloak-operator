use either::Either;

pub enum Either2<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> From<Either<L, R>> for Either2<L, R> {
    fn from(e: Either<L, R>) -> Self {
        match e {
            Either::Left(l) => Either2::Left(l),
            Either::Right(r) => Either2::Right(r),
        }
    }
}

impl<L, R> Into<Either<L, R>> for Either2<L, R> {
    fn into(self) -> Either<L, R> {
        match self {
            Either2::Left(l) => Either::Left(l),
            Either2::Right(r) => Either::Right(r),
        }
    }
}
