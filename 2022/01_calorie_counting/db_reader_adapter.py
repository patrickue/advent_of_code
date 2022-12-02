import sqlite3
import typing

from ports import InputPort


class DBReaderAdapter(InputPort):
    db_connection: typing.Any

    def __init__(self, db_file_name: typing.Any):
        self.db_connection = sqlite3.connect(db_file_name)
        self.cursor = self.db_connection.execute(
            "select length, width, height from dimension"
        )

    def __iter__(self) -> typing.Iterator[typing.Any]:
        return self

    def __next__(self) -> typing.Any:
        return self.cursor.fetchone()
