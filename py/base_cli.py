#!/usr/bin/env python3

# pyre-strict

import asyncio
import logging
import sys
from typing import Any, Union

import click

__version__ = "0.6.9"
LOG: logging.Logger = logging.getLogger(__name__)


def _handle_debug(
    ctx: Union[click.core.Context, None],
    param: Union[click.core.Option, click.core.Parameter, None],
    debug: Union[bool, int, str],
) -> Union[bool, int, str]:
    """Turn on debugging if asked otherwise INFO default"""
    log_level = logging.DEBUG if debug else logging.INFO
    logging.basicConfig(
        format="[%(asctime)s] %(levelname)s: %(message)s (%(filename)s:%(lineno)d)",
        level=log_level,
    )
    return debug


async def async_main(debug: bool) -> int:
    return 0


# Some typing stub issue I can't work out - https://github.com/pallets/click/issues/2558
# Fix for 8.1.6 dosen't seem to fix me - To look into
@click.command(context_settings={"help_option_names": ["-h", "--help"]})  # type: ignore
@click.option(
    "--debug",
    is_flag=True,
    callback=_handle_debug,
    show_default=True,
    help="Turn on debug logging",
)
@click.pass_context
def main(ctx: click.core.Context, **kwargs: Any) -> None:
    LOG.debug(f"Starting {sys.argv[0]}")
    ctx.exit(asyncio.run(async_main(**kwargs)))


if __name__ == "__main__":  # pragma: no cover
    main()
