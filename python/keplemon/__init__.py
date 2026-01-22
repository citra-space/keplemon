# flake8: noqa
from __future__ import annotations

import os
import logging
from importlib import resources

_logger = logging.getLogger(__name__)


def _set_asset_directory() -> None:

    asset_dir = os.getenv("SAAL_ASSET_DIRECTORY")
    if asset_dir is None:
        pkg_dir = resources.files("keplemon")
        _logger.debug("Setting SAAL_ASSET_DIRECTORY to %s", pkg_dir)
        os.environ.setdefault("SAAL_ASSET_DIRECTORY", str(pkg_dir))
    elif not os.path.exists(asset_dir):
        raise FileNotFoundError(f"SAAL_ASSET_DIRECTORY '{asset_dir}' does not exist.")


_set_asset_directory()

from keplemon._keplemon import (  # type: ignore
    get_thread_count,
    set_thread_count,
)

__all__ = [
    "get_thread_count",
    "set_thread_count",
]
