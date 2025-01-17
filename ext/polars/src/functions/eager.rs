use magnus::RArray;
use polars::{functions, time};
use polars_core::datatypes::{TimeUnit, TimeZone};
use polars_core::prelude::{DataFrame, IntoSeries};

use crate::conversion::{get_df, get_series, Wrap};
use crate::error::RbPolarsErr;
use crate::prelude::{ClosedWindow, Duration};
use crate::{RbDataFrame, RbResult, RbSeries};

pub fn concat_df(seq: RArray) -> RbResult<RbDataFrame> {
    use polars_core::error::PolarsResult;

    let mut iter = seq.each();
    let first = iter.next().unwrap()?;

    let first_rdf = get_df(first)?;
    let identity_df = first_rdf.slice(0, 0);

    let mut rdfs: Vec<PolarsResult<DataFrame>> = vec![Ok(first_rdf)];

    for item in iter {
        let rdf = get_df(item?)?;
        rdfs.push(Ok(rdf));
    }

    let identity = Ok(identity_df);

    let df = rdfs
        .into_iter()
        .fold(identity, |acc: PolarsResult<DataFrame>, df| {
            let mut acc = acc?;
            acc.vstack_mut(&df?)?;
            Ok(acc)
        })
        .map_err(RbPolarsErr::from)?;

    Ok(df.into())
}

pub fn concat_series(seq: RArray) -> RbResult<RbSeries> {
    let mut iter = seq.each();
    let first = iter.next().unwrap()?;

    let mut s = get_series(first)?;

    for res in iter {
        let item = res?;
        let item = get_series(item)?;
        s.append(&item).map_err(RbPolarsErr::from)?;
    }
    Ok(s.into())
}

pub fn date_range(
    start: i64,
    stop: i64,
    every: String,
    closed: Wrap<ClosedWindow>,
    name: String,
    tu: Wrap<TimeUnit>,
    tz: Option<TimeZone>,
) -> RbResult<RbSeries> {
    let date_range = time::date_range_impl(
        &name,
        start,
        stop,
        Duration::parse(&every),
        closed.0,
        tu.0,
        tz.as_ref(),
    )
    .map_err(RbPolarsErr::from)?;
    Ok(date_range.into_series().into())
}

pub fn diag_concat_df(seq: RArray) -> RbResult<RbDataFrame> {
    let mut dfs = Vec::new();
    for item in seq.each() {
        dfs.push(get_df(item?)?);
    }
    let df = functions::diag_concat_df(&dfs).map_err(RbPolarsErr::from)?;
    Ok(df.into())
}

pub fn hor_concat_df(seq: RArray) -> RbResult<RbDataFrame> {
    let mut dfs = Vec::new();
    for item in seq.each() {
        dfs.push(get_df(item?)?);
    }
    let df = functions::hor_concat_df(&dfs).map_err(RbPolarsErr::from)?;
    Ok(df.into())
}
