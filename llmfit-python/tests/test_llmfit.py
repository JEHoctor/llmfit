"""Tests for the llmfit umbrella package and its workspace members."""

from __future__ import annotations

import re
import subprocess
from pathlib import Path

import pytest

import llmfit
from llmfit import find_llmfit_bin


def test_find_llmfit_bin_raises_when_missing(monkeypatch: pytest.MonkeyPatch) -> None:
    """Tests that find_llmfit_bin raises a BinaryNotFoundError when the binary is missing."""
    monkeypatch.setattr(Path, "is_file", lambda _: False)
    with pytest.raises(llmfit.BinaryNotFoundError):
        find_llmfit_bin()


def test_binary_path_is_path() -> None:
    """Tests that find_llmfit_bin returns a Path object."""
    assert isinstance(find_llmfit_bin(), Path)


@pytest.mark.rust_integration
def test_binary_runs() -> None:
    """Tests that the llmfit binary runs successfully."""
    result = subprocess.run([find_llmfit_bin(), "--help"], capture_output=True, check=False)
    assert result.returncode == 0


def test_version() -> None:
    """Tests that llmfit.__version__ is a valid semantic version."""
    assert re.match(r"^\d+\.\d+\.\d+$", llmfit.__version__) is not None


@pytest.mark.rust_integration
def test_detect_system_returns_system_info() -> None:
    """Tests that detect_system returns a SystemInfo with plausible values."""
    info = llmfit.detect_system()
    assert info.total_ram_gb > 0
    assert info.cpu_cores > 0
    assert isinstance(info.cpu_name, str)
    assert info.cpu_name
    assert isinstance(info.has_gpu, bool)
    assert isinstance(info.gpus, list)


@pytest.mark.rust_integration
def test_system_info_repr() -> None:
    """Tests that SystemInfo has a useful repr."""
    info = llmfit.detect_system()
    assert "SystemInfo" in repr(info)
