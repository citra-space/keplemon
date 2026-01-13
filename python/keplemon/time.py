from keplemon._keplemon.time import (  # type: ignore
    TimeSpan,
    TimeComponents,
    Epoch,
)
import requests  # type: ignore
from datetime import datetime
from keplemon._keplemon.enums import TimeSystem  # type: ignore

__all__ = [
    "TimeSpan",
    "TimeComponents",
    "Epoch",
    "request_time_constants_update",
]


def request_time_constants_update(output_path: str) -> None:
    finals = requests.get("https://maia.usno.navy.mil/ser7/finals.all").text.splitlines()

    leap_seconds = requests.get("https://maia.usno.navy.mil/ser7/tai-utc.dat").text.splitlines()

    month_map = {
        "JAN": 1,
        "FEB": 2,
        "MAR": 3,
        "APR": 4,
        "MAY": 5,
        "JUN": 6,
        "JUL": 7,
        "AUG": 8,
        "SEP": 9,
        "OCT": 10,
        "NOV": 11,
        "DEC": 12,
    }
    leap_second_map = {}
    for line in leap_seconds:
        yy = int(line[3:5].strip())
        mm = month_map[line[6:9]]
        dd = int(line[10:12].strip())
        time_comps = TimeComponents(yy, mm, dd, 0, 0, 0)
        current_epoch = Epoch.from_time_components(time_comps, TimeSystem.UTC)
        ds50 = int(current_epoch.days_since_1950)
        leap = float(line[38:48].strip())
        leap_second_map[ds50] = leap

    time_constant_lines: list[str] = []
    sorted_leap_seconds = sorted(leap_second_map, reverse=True)
    ut1 = 0.0
    prev_ut1_utc = 0.0
    prev_time = 0
    prev_leap = 0.0
    for line in finals:

        if len(line.strip()) < 68:
            break

        yy = int(line[0:2].strip())
        mm = int(line[2:4].strip())
        dd = int(line[4:6].strip())
        if yy < 50:
            yyyy = 2000 + yy
        else:
            yyyy = 1900 + yy
        ymd = datetime(yyyy, mm, dd).strftime("%d-%b-%y")

        time_comps = TimeComponents(yy, mm, dd, 0, 0, 0)
        current_epoch = Epoch.from_time_components(time_comps, TimeSystem.UTC)
        ds50 = int(current_epoch.days_since_1950)
        leap = 0.0
        for leap_ds50 in sorted_leap_seconds:
            if ds50 >= leap_ds50:
                leap = leap_second_map[leap_ds50]
                break

        ut1 = float(line[58:68].strip())
        if prev_time == 0:
            ut1_r = 0.0
        else:
            ut1_r = (ut1 - prev_ut1_utc) / (ds50 - prev_time)
            ut1_r = (ut1_r - (leap - prev_leap)) * 1e3

        p_x = float(line[18:27].strip())
        p_y = float(line[37:46].strip())

        doy = current_epoch.day_of_year
        time_constant_lines.append(
            f" {yy:02d}  {doy:03.0f}   {ymd} {leap:3.0f}  {ut1:8.5f}  {ut1_r:6.3f}   {p_x:7.4f}   {p_y:7.4f}\n"
        )
        prev_time = ds50
        prev_ut1_utc = ut1
        prev_leap = leap

    with open(output_path, "w") as f:
        f.writelines(time_constant_lines)
