export class InputStream {
    constructor(iterable, leftCol = 0) {
        this.queue = iterable;
        this.position = 0;
        if (typeof iterable === "string") {
            this.leftCol = leftCol;
            this.row = 1;
            this.col = leftCol;
        }
    }
    
    eof(throwError = true) {
        const endOfFile = this.position >= this.queue.length;
        if (throwError && endOfFile) {
            this.yeet("Reached end of queue");
        } else return endOfFile;
    }
    
    peek(throwError = false, distance = 0) {
        if (this.eof(throwError)) return null;
        return this.queue[this.position + distance];
    }
    
    next() {
        const item = this.queue[this.position++];
        if (typeof this.queue === "string") {
            this.col++;
            if (item === "\n") {
                this.row++;
                this.col = this.leftCol;
            }
        }
        return item;
    }
    
    until(condition, including = true) {
        let res = [];
        while (!this.eof(false) && !condition(this.peek())) {
            res.push(this.next());
        }
        if (including) {
            if (this.eof(false)) {
                this.yeet("Expected ';', found EOF");
            }
            this.skip();
        }
        return res;
    }
    
    skip() {
        this.next();
    }
    
    yeet(msg) {
        if (typeof this.queue === "string")
            throw new Error(`${msg} (${this.row}:${this.col})`);
        else
            throw new Error(`${msg} (${this.position})`);
    }
}