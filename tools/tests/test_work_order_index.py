import contextlib
import io
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_docs


class WorkOrderIndexTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name).resolve()
        self.orders = root / check_docs.WORK_ORDERS
        (root / "conformance").mkdir()
        (root / "conformance/manifest.yaml").touch()
        root_patch = patch.object(check_docs, "ROOT", root)
        root_patch.start()
        self.addCleanup(root_patch.stop)

    def write(self, name, text):
        path = self.orders / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        return path

    def invoke(self, *args):
        stdout, stderr = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            code = check_docs.main(args)
        return code, stdout.getvalue(), stderr.getvalue()

    def test_work_order_index_from_first_lines(self):
        readme = "# Work orders\n\n| Order | What |\n| --- | --- |\n| 048a7 | Planned |\n"
        readme_path = self.write("README.md", readme)
        self.write("027-third.md", "# Work order 027: Third order\n")
        self.write("004-first.md", "# Work order 004: First order\n\nBody text\n")
        self.write("011a-second.md", "# Work order 011a (Plan A): Second order: a label\n")
        self.write("verify.md", "# Verify\n")
        self.write("nested/006-ignored.md", "# No work order heading\n")

        index = (
            "| Order | What |\n"
            "| --- | --- |\n"
            "| [004](004-first.md) | First order |\n"
            "| [011a](011a-second.md) | Second order: a label |\n"
            "| [027](027-third.md) | Third order |\n"
        )
        self.assertEqual(self.invoke("--work-orders"), (0, index, ""))
        self.write("031-fourth.md", "# Work order 031: Fourth order\n")
        self.assertEqual(self.invoke("--walk", "--work-orders"),
                         (0, index + "| [031](031-fourth.md) | Fourth order |\n", ""))
        self.assertEqual(readme_path.read_text(encoding="utf-8"), readme)
        for args in ((), ("--walk",)):
            code, output, errors = self.invoke(*args)
            self.assertEqual((code, output.splitlines()[-1], errors), (0, "Errors: 0", ""))
        self.assertEqual(self.invoke("--unknown")[0], 1)

    def test_work_order_index_refuses_bad_lines_and_rows(self):
        bad = {
            "001-missing-heading.md": "\n# Work order 001: Too late",
            "002-wrong-id.md": "# Work order 003: Wrong ID",
            "003-empty-title.md": "# Work order 003: ",
            "004-pipe.md": "# Work order 004: Left | right",
            "005-b-duplicate.md": "# Work order 005: Duplicate",
        }
        for name, text in bad.items():
            self.write(name, text + "\n")
        self.write("005-a-original.md", "# Work order 005: Original\n")
        code, stdout, errors = self.invoke("--work-orders")
        self.assertEqual((code, stdout), (1, ""))
        self.assertEqual([line.split(":", 1)[0] for line in errors.splitlines()],
                         ["docs/work-orders/" + name for name in bad])

        for table in (
            "| Order | What |\n| --- | --- |\n| [005](005-a-original.md) | Stale row |\n",
            "Order | What\n--- | ---\n[005][work] | Stale row\n\n[work]: ./005-a-original.md\n",
        ):
            with self.subTest(table=table):
                self.write("README.md", "# Work orders\n\n" + table)
                code, stdout, errors = self.invoke()
                self.assertEqual((code, stdout.splitlines()[-1]), (1, "Errors: 6"))
                self.assertEqual([line.split(":", 1)[0] for line in errors.splitlines()],
                                 ["docs/work-orders/" + name for name in (*bad, "README.md")])

    def test_order_table_links_report_one_readme_error(self):
        self.write("238-derived-index.md", "# Work order 238: Derived index\n")
        for link in ("[238](238-derived-index.md)", "[238][]", "[238]", "[Index][238]"):
            with self.subTest(link=link):
                self.write("README.md", "# Work orders\n\nOrder | What\n--- | ---\n"
                                        f"{link} | Stale row\n"
                                        "\n[238]: ./238-derived-index.md\n")
                code, stdout, errors = self.invoke()
                self.assertEqual((code, stdout.splitlines()[-1]), (1, "Errors: 1"))
                self.assertEqual(errors.splitlines(), [
                    "docs/work-orders/README.md:5: generate work order rows with --work-orders",
                ])

    def test_pipe_led_prose_outside_order_table_is_allowed(self):
        self.write("238-derived-index.md", "# Work order 238: Derived index\n")
        self.write("README.md", "# Work orders\n\n"
                                "| See [238] for the change.\n\n"
                                "Order | What\n--- | ---\n048a7 | Planned\n\n"
                                "| See [238](238-derived-index.md).\n\n"
                                "| [238] | Missing separator | Columns |\n| --- | --- |\n\n"
                                "[238]: ./238-derived-index.md\n")
        code, stdout, errors = self.invoke()
        self.assertEqual((code, stdout.splitlines()[-1], errors), (0, "Errors: 0", ""))
