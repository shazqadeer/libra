// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

macro_rules! cfg_async {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "async")]
            #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
            $item
        )*
    }
}

macro_rules! cfg_blocking {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "blocking")]
            #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
            $item
        )*
    }
}

macro_rules! cfg_async_or_blocking {
    ($($item:item)*) => {
        $(
            #[cfg(any(
                feature = "async",
                feature = "blocking",
            ))]
            #[cfg_attr(docsrs, doc(cfg(any(
                feature = "async",
                feature = "blocking",
            ))))]
            $item
        )*
    }
}
