#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::{
    pin::Pin, task::{Context, Poll},
    time::Duration,
};
use tokio::sync::{mpsc, oneshot};
async fn select_basic_usage() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        let _ = tx1.send("one");
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    {
        #[doc(hidden)]
        mod __tokio_select_util {
            pub(super) enum Out<_0, _1> {
                _0(_0),
                _1(_1),
                Disabled,
            }
            pub(super) type Mask = u8;
        }
        use ::tokio::macros::support::Future;
        use ::tokio::macros::support::Pin;
        use ::tokio::macros::support::Poll::{Ready, Pending};
        const BRANCHES: u32 = 2;
        let mut disabled: __tokio_select_util::Mask = Default::default();
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 0;
            disabled |= mask;
        }
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 1;
            disabled |= mask;
        }
        let mut output = {
            let futures_init = (rx1, rx2);
            let mut futures = (
                ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
            );
            let mut futures = &mut futures;
            ::tokio::macros::support::poll_fn(|cx| {
                    match ::tokio::macros::support::poll_budget_available(cx) {
                        ::core::task::Poll::Ready(t) => t,
                        ::core::task::Poll::Pending => {
                            return ::core::task::Poll::Pending;
                        }
                    };
                    let mut is_pending = false;
                    let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                    for i in 0..BRANCHES {
                        let branch;
                        #[allow(clippy::modulo_one)]
                        {
                            branch = (start + i) % BRANCHES;
                        }
                        match branch {
                            #[allow(unreachable_code)]
                            0 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_0(out));
                            }
                            #[allow(unreachable_code)]
                            1 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (_, fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_1(out));
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!(
                                            "reaching this means there probably is an off by one bug",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                    if is_pending {
                        Pending
                    } else {
                        Ready(__tokio_select_util::Out::Disabled)
                    }
                })
                .await
        };
        match output {
            __tokio_select_util::Out::_0(val) => {
                {
                    ::std::io::_print(
                        format_args!("rx1 completed first with {0:?}\n", val),
                    );
                };
            }
            __tokio_select_util::Out::_1(val) => {
                {
                    ::std::io::_print(
                        format_args!("rx2 completed first with {0:?}\n", val),
                    );
                };
            }
            __tokio_select_util::Out::Disabled => {
                ::core::panicking::panic_fmt(
                    format_args!("all branches are disabled and there is no else branch"),
                );
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("failed to match bind"),
                    ),
                );
            }
        }
    }
}
async fn some_operation() -> &'static str {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "some operation"
}
async fn cancel_future() {
    let (tx1, mut rx1): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    let (tx2, mut rx2): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    tokio::spawn(async move {
        {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1> {
                    _0(_0),
                    _1(_1),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 2;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            let mut output = {
                let futures_init = (some_operation(), async {});
                let mut futures = (
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
                );
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        match ::tokio::macros::support::poll_budget_available(cx) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        };
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        val => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        _ => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(val) => {
                    let _ = tx1.send(val).await;
                }
                __tokio_select_util::Out::_1(_) => {
                    drop(tx1);
                }
                __tokio_select_util::Out::Disabled => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "all branches are disabled and there is no else branch",
                        ),
                    );
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        }
    });
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        let _ = tx2.send("two").await;
    });
    {
        #[doc(hidden)]
        mod __tokio_select_util {
            pub(super) enum Out<_0, _1> {
                _0(_0),
                _1(_1),
                Disabled,
            }
            pub(super) type Mask = u8;
        }
        use ::tokio::macros::support::Future;
        use ::tokio::macros::support::Pin;
        use ::tokio::macros::support::Poll::{Ready, Pending};
        const BRANCHES: u32 = 2;
        let mut disabled: __tokio_select_util::Mask = Default::default();
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 0;
            disabled |= mask;
        }
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 1;
            disabled |= mask;
        }
        let mut output = {
            let futures_init = (rx1.recv(), rx2.recv());
            let mut futures = (
                ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
            );
            let mut futures = &mut futures;
            ::tokio::macros::support::poll_fn(|cx| {
                    match ::tokio::macros::support::poll_budget_available(cx) {
                        ::core::task::Poll::Ready(t) => t,
                        ::core::task::Poll::Pending => {
                            return ::core::task::Poll::Pending;
                        }
                    };
                    let mut is_pending = false;
                    let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                    for i in 0..BRANCHES {
                        let branch;
                        #[allow(clippy::modulo_one)]
                        {
                            branch = (start + i) % BRANCHES;
                        }
                        match branch {
                            #[allow(unreachable_code)]
                            0 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_0(out));
                            }
                            #[allow(unreachable_code)]
                            1 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (_, fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_1(out));
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!(
                                            "reaching this means there probably is an off by one bug",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                    if is_pending {
                        Pending
                    } else {
                        Ready(__tokio_select_util::Out::Disabled)
                    }
                })
                .await
        };
        match output {
            __tokio_select_util::Out::_0(val) => {
                {
                    ::std::io::_print(
                        format_args!("rx1 completed first with {0:?}\n", val),
                    );
                };
            }
            __tokio_select_util::Out::_1(val) => {
                {
                    ::std::io::_print(
                        format_args!("rx2 completed first with {0:?}\n", val),
                    );
                };
            }
            __tokio_select_util::Out::Disabled => {
                ::core::panicking::panic_fmt(
                    format_args!("all branches are disabled and there is no else branch"),
                );
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("failed to match bind"),
                    ),
                );
            }
        }
    }
}
async fn my_select_fn() {
    struct MySelect {
        rx1: mpsc::Receiver<&'static str>,
        rx2: mpsc::Receiver<&'static str>,
    }
    impl Future for MySelect {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Poll::Ready(val) = self.rx1.poll_recv(cx) {
                {
                    ::std::io::_print(
                        format_args!("rx1 completed first with {0:?}\n", val),
                    );
                };
                return Poll::Ready(());
            } else if let Poll::Ready(val) = self.rx2.poll_recv(cx) {
                {
                    ::std::io::_print(
                        format_args!("rx2 completed first with {0:?}\n", val),
                    );
                };
                return Poll::Ready(());
            }
            Poll::Pending
        }
    }
    let (tx1, rx1): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    let (tx2, rx2): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    tokio::spawn(async move {
        {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1> {
                    _0(_0),
                    _1(_1),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 2;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            let mut output = {
                let futures_init = (some_operation(), async {});
                let mut futures = (
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
                );
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        match ::tokio::macros::support::poll_budget_available(cx) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        };
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        val => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        _ => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(val) => {
                    let _ = tx1.send(val).await;
                }
                __tokio_select_util::Out::_1(_) => {
                    drop(tx1);
                }
                __tokio_select_util::Out::Disabled => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "all branches are disabled and there is no else branch",
                        ),
                    );
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        }
    });
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        let _ = tx2.send("two").await;
    });
    MySelect { rx1, rx2 }.await;
}
async fn pattern_match() {
    let (tx1, mut rx1): (mpsc::Sender<&str>, _) = mpsc::channel(128);
    let (tx2, mut rx2): (mpsc::Sender<&str>, _) = mpsc::channel(128);
    tokio::spawn(async move {
        tx1.closed().await;
        tx2.closed().await;
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    {
        #[doc(hidden)]
        mod __tokio_select_util {
            pub(super) enum Out<_0, _1> {
                _0(_0),
                _1(_1),
                Disabled,
            }
            pub(super) type Mask = u8;
        }
        use ::tokio::macros::support::Future;
        use ::tokio::macros::support::Pin;
        use ::tokio::macros::support::Poll::{Ready, Pending};
        const BRANCHES: u32 = 2;
        let mut disabled: __tokio_select_util::Mask = Default::default();
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 0;
            disabled |= mask;
        }
        if !true {
            let mask: __tokio_select_util::Mask = 1 << 1;
            disabled |= mask;
        }
        let mut output = {
            let futures_init = (rx1.recv(), rx2.recv());
            let mut futures = (
                ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
            );
            let mut futures = &mut futures;
            ::tokio::macros::support::poll_fn(|cx| {
                    match ::tokio::macros::support::poll_budget_available(cx) {
                        ::core::task::Poll::Ready(t) => t,
                        ::core::task::Poll::Pending => {
                            return ::core::task::Poll::Pending;
                        }
                    };
                    let mut is_pending = false;
                    let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                    for i in 0..BRANCHES {
                        let branch;
                        #[allow(clippy::modulo_one)]
                        {
                            branch = (start + i) % BRANCHES;
                        }
                        match branch {
                            #[allow(unreachable_code)]
                            0 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_0(out));
                            }
                            #[allow(unreachable_code)]
                            1 => {
                                let mask = 1 << branch;
                                if disabled & mask == mask {
                                    continue;
                                }
                                let (_, fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                let out = match Future::poll(fut, cx) {
                                    Ready(out) => out,
                                    Pending => {
                                        is_pending = true;
                                        continue;
                                    }
                                };
                                disabled |= mask;
                                #[allow(unused_variables)] #[allow(unused_mut)]
                                match &out {
                                    val => {}
                                    _ => continue,
                                }
                                return Ready(__tokio_select_util::Out::_1(out));
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!(
                                            "reaching this means there probably is an off by one bug",
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                    if is_pending {
                        Pending
                    } else {
                        Ready(__tokio_select_util::Out::Disabled)
                    }
                })
                .await
        };
        match output {
            __tokio_select_util::Out::_0(val) => {
                {
                    ::std::io::_print(format_args!("Got {0:?} from rx1\n", val));
                };
            }
            __tokio_select_util::Out::_1(val) => {
                {
                    ::std::io::_print(format_args!("Got {0:?} from rx2\n", val));
                };
            }
            __tokio_select_util::Out::Disabled => {
                {
                    ::std::io::_print(format_args!("Both channels closed\n"));
                };
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("failed to match bind"),
                    ),
                );
            }
        }
    }
}
async fn loop_select() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);
    tokio::spawn(async move {
        let _ = tx1.send("1").await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let _ = tx2.send("2").await;
        let _ = tx3.send("3").await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    #[allow(clippy::never_loop)]
    loop {
        let msg = {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1, _2> {
                    _0(_0),
                    _1(_1),
                    _2(_2),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 3;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 2;
                disabled |= mask;
            }
            let mut output = {
                let futures_init = (rx1.recv(), rx2.recv(), rx3.recv());
                let mut futures = (
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.2),
                );
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        match ::tokio::macros::support::poll_budget_available(cx) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        };
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        Some(msg) => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        Some(msg) => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                #[allow(unreachable_code)]
                                2 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, _, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        Some(msg) => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_2(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(Some(msg)) => msg,
                __tokio_select_util::Out::_1(Some(msg)) => msg,
                __tokio_select_util::Out::_2(Some(msg)) => msg,
                __tokio_select_util::Out::Disabled => break,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        };
        {
            ::std::io::_print(format_args!("Got {0:?}\n", msg));
        };
    }
    {
        ::std::io::_print(format_args!("All channels have been closed.\n"));
    };
}
async fn action(input: Option<i32>) -> Option<i32> {
    input
}
async fn resume_async() {
    let (tx, mut rx): (mpsc::Sender<i32>, _) = mpsc::channel(128);
    let mut done = false;
    let operation = action(None);
    let mut operation = operation;
    #[allow(unused_mut)]
    let mut operation = unsafe {
        ::tokio::macros::support::Pin::new_unchecked(&mut operation)
    };
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(2).await;
        let _ = tx.send(3).await;
    });
    loop {
        {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1> {
                    _0(_0),
                    _1(_1),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 2;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !!done {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            let mut output = {
                let futures_init = (&mut operation, rx.recv());
                let mut futures = (
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
                );
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        match ::tokio::macros::support::poll_budget_available(cx) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        };
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        res => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        Some(val) => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(res) => {
                    done = true;
                    if let Some(val) = res {
                        {
                            ::std::io::_print(format_args!("GOT = {0}\n", val));
                        };
                        return;
                    }
                }
                __tokio_select_util::Out::_1(Some(val)) => {
                    if val % 2 == 0 {
                        operation.set(action(Some(val)));
                        done = false;
                    }
                }
                __tokio_select_util::Out::Disabled => break,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        }
    }
}
fn main() {
    let body = async {
        resume_async().await;
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
