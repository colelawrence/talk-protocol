/**
 * Ref ties multiple pieces of information together
 * refs might also be used to track when claims have actually been updated
 */
type Ref = {
    rid: string,
}
/** Create some universally unique symbol */
type Entity = { id: string }

type NewStmt = {
    template: string,
    values: any[],
}

type NewQuery = {
    query: string[],
    /** you may use `_ claims (you) points ("up") at /target/` and provide (you) as a definition here. */
    defs?: {
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
     * wait might return with match = null, indicating that an update invalidated the match
     */
    wait(qId: QId, sync: string): { match: NewMatch | null, sync: string }
}
