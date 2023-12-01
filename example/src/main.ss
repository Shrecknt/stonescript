import utils::summon_marker;

static test: int = 1 + a / (2 * cat);
static function main(): int {
    static loaded: int = test;
    {
        // This is a comment
        let scoped: float = 0.0;
    }

    for (let i: int = 0; i < 5; i = i + 1;) {
        summon_marker();
    }

    $command("say loading!");

    unsafe {
        eval("say this is being run from an unchecked context o_o");
    }

    if (test == 1) {
        $command("say 1!");
    } else if (test == 2) {
        $command("say 2!");
    } else {
        $command("say not 1 or 2 :<");
    }

    return 1;
}

static function tick(): void {
    test = test + 1;
}