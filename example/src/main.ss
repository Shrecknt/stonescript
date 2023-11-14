static test: int = 1 + a / (2 * cat);
test = testing;

static function main(): int {
    static loaded: int = test;
    {
        // This is a comment
        let scoped: float = 0.0;
    }

    for (let i: int = 0; i < 5; i = i + 1;) {
        tick();
    }

    command("say loading!");

    if (1) {
        command("say 1!");
    } else if (2) {
        command("say 2!");
    } else {
        command("say not 1 or 2 :<");
    }

    return 1;
}

static function tick(): void {
    command("say tick!");
}