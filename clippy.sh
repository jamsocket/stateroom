#!/bin/sh

cargo clippy -- \
    -W clippy::pedantic \
    -A clippy::doc_markdown \
    -W clippy::unwrap_used \
    -A clippy::module_name_repetitions \
    -A clippy::missing_errors_doc
