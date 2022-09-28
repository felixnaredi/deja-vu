export namespace History {
  export enum EntryState {
    Unseen,
    Seen,
  }

  export class Entry<T> {
    public value: T;
    public actualState: EntryState;
    public answeredState: null | EntryState;

    constructor(value: T, actualState: EntryState) {
      this.value = value;
      this.actualState = actualState;
      this.answeredState = null;
    }
  }
}

export class HistoryTracker<T> {
  public entries: Array<History.Entry<T>> = [];

  public set current(value: T) {
    if (this.entries.some((entry) => entry.value == value)) {
      this.entries.push(new History.Entry(value, History.EntryState.Seen));
    } else {
      this.entries.push(new History.Entry(value, History.EntryState.Unseen));
    }
  }

  public set answer(state: History.EntryState) {
    this.entries[this.entries.length - 1].answeredState = state;
  }
}
