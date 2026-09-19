use try_partialord::{InvalidOrderError, TryMinMax};

use crate::calc_traits::Absolutable;

pub trait StatisticsOrderedPartial<T> {
    fn try_min_stat(&self) -> Result<Option<&T>, InvalidOrderError>;
    fn try_max_stat(&self) -> Result<Option<&T>, InvalidOrderError>;
}

pub trait StatisticsOrderedAbsPartial<T> {
    fn try_abs_min(&self) -> Result<Option<T>, InvalidOrderError>;
    fn try_abs_max(&self) -> Result<Option<T>, InvalidOrderError>;
}

impl<T> StatisticsOrderedPartial<T> for Vec<T>
where
    T: PartialOrd,
{
    fn try_min_stat(&self) -> Result<Option<&T>, InvalidOrderError> {
        self.iter().try_min()
    }
    fn try_max_stat(&self) -> Result<Option<&T>, InvalidOrderError> {
        self.iter().try_max()
    }
}

impl<T> StatisticsOrderedAbsPartial<T> for Vec<T>
where
    T: PartialOrd + Absolutable,
{
    fn try_abs_min(&self) -> Result<Option<T>, InvalidOrderError> {
        self.iter().try_min().map(|val| val.map(Absolutable::abs))
    }
    fn try_abs_max(&self) -> Result<Option<T>, InvalidOrderError> {
        self.iter().try_max().map(|val| val.map(Absolutable::abs))
    }
}
