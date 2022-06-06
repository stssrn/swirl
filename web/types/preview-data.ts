import { FileType } from "./file-type";

export interface PreviewData {
  type: FileType;
  filePath: string;
  content?: string;
  isMarkdown?: boolean;
}
