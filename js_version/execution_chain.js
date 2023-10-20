export class ExecutionChain {
    constructor() {
        this.chain = [];
    }
    
    then(callback, error) {
        this.chain.push({ callback, error });
        return this;
    }
    
    execute(...argumentList) {
        for (let callback of this.chain) {
            try {
                argumentList = callback.callback(...argumentList) || [];
                if (!Array.isArray(argumentList)) argumentList = [argumentList];
            } catch (err) {
                callback.error(err);
                return;
            }
        }
    }
}