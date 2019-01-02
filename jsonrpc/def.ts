/** Ref ties multiple pieces of information together */
type Ref = {
    rid: string,
}
type Entity = { id: string }

type NewStmt = {
    template: string,
    values: any[],
}

type NewQuery = {
    query: string[],
    vars?: {
        [identifier: string]: any,
    },
}

type QId = string

let qss = [
    `_ claims /x/ points ("up") at ({"__type": "Page", "__ref": "@3214"})`,
    `_ claims /x/ points ("up") at (you)`,
]

type NewMatch = {
    qId: QId,
    ref: Ref,
    values: {
        [identifier: string]: any,
    },
}

interface DB {
    listen(ref: Ref, query: NewQuery): QId;
    insert(ref: Ref, statement: NewStmt): void;
    new_entity(ref: Ref): Entity;

    /**
     * wait should be called when the client is ready to accept updates
     * wait does not return immediately
     */
    wait(qId: QId, sync: string): { match: NewMatch, sync: string }
}

interface Client {
    new(db: DB, you: Ref): void
    dispose(): void
}