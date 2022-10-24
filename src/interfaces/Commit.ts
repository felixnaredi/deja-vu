import SeenUnseen from "@/interfaces/SeenUnseen";

interface Commit {
  element: () => string;
  actual: () => SeenUnseen;
  guess: () => SeenUnseen;
  correct: () => boolean;
}

export default Commit;
