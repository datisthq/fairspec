from fairspec_terminal.program import program, register_commands


def main() -> None:
    register_commands()
    program()
