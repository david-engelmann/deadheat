"""The panel join over recorded adjudication records.

Two records of one race, each carrying at most its single DH-5
adjudication entry, join into one panel record holding the
contributed entries under the field panel, in the order the
records were given. The join is a pure function of the two
loaded records, it never raises, and it never mutates its
inputs. A pair that fails the join returns no record and is
never merged by force. The obligation this module carries is
DH-10 in CONTRACT.md, in the Round 4 section, and the panel
shape and reading it assembles are DH-9.
"""


def join_panel_records(first, second):
    """Join two loaded records of one race into a panel record.

    Returns a pair of the panel record and the defect list. The
    two records must name the same task and hold the same
    attempts, as loaded. Each record contributes its adjudication
    entry when it carries one and contributes absence when it
    carries none, and a pair holding no entry between them is
    refused. A record carrying a panel is refused, because its
    entries already stand joined and the join assembles a panel
    from single entries alone, and where a panel stands in the
    pair the pair draws no defect for holding no entry, since the
    refused panel is where the entries stand. The refusal is
    FIX-DH-14 in CONTRACT.md, amending DH-10. Two entries naming
    the same adjudicator are refused, because a panel stands one
    entry per adjudicating model. The panel record holds the task
    and the attempts of the two records and the contributed
    entries under the field panel, and it loads under DH-9 with
    no defects.
    """
    defects = []
    if first["task"] != second["task"]:
        defects.append("the two records name different tasks")
    if first["attempts"] != second["attempts"]:
        defects.append("the two records hold different attempts")
    panel_seen = "panel" in first or "panel" in second
    if panel_seen:
        defects.append("one of the two records carries a panel")
    entries = []
    for record in (first, second):
        entry = record.get("adjudication")
        if entry is not None:
            entries.append(entry)
    if not entries and not panel_seen:
        defects.append(
            "the two records hold no adjudication entry between them"
        )
    elif (
        len(entries) == 2
        and entries[0]["adjudicator"] == entries[1]["adjudicator"]
    ):
        defects.append("the two entries name the same adjudicator")
    if defects:
        return None, defects
    return {
        "task": first["task"],
        "attempts": first["attempts"],
        "panel": entries,
    }, []
