# try_zip

A variant of `zip` that can handle a shorter/optional second iterator.

Given `(0..10).zip(0..5)`, `zip` would stop after the shorter iterator `0..5` ends.

`TryZip::try_zip(0..10, Some(0..5))`, however, will continue, using `None` for the second value of the tuple. (The tuple returned by `TryZip` is of the type `(A::Item, Option<B::Item>)`)

This crate was written for easily parsing IRC join messages.