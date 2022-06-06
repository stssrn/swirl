export interface TreeEntry {
  id: string;
  size?: number;
  name: string;
  entries?: TreeEntry[];
}

export interface TreeResult {
  node: TreeEntry;
  isFile: boolean;
}
