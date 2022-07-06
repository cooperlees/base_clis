#!/usr/bin/env python3

import asyncio
import unittest

from click.testing import CliRunner

from base_cli import _handle_debug, async_main, main


class TestCLI(unittest.TestCase):
    def test_async_main(self) -> None:
        self.assertEqual(0, asyncio.run(async_main(True)))

    def test_debug_output(self) -> None:
        self.assertTrue(_handle_debug(None, None, True))

    def test_help(self) -> None:
        runner = CliRunner()
        result = runner.invoke(main, ["--help"])
        assert result.exit_code == 0


if __name__ == "__main__":  # pragma: no cover
    unittest.main()
