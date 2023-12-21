import { TAuthorData } from "./author";

export type TBookData = {
  id: number;
  title: string;
  author: TAuthorData;
  year: number;
};
