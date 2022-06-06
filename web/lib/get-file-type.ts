import { FileType } from "../types/file-type";

export default function getFileType(filename: string): FileType {
  const filenameSegments = filename.split(".");
  const extension = filenameSegments[filenameSegments.length - 1].toUpperCase();

  switch (extension) {
    case "APNG":
    case "AVIF":
    case "BMP":
    case "CUR":
    case "GIF":
    case "ICO":
    case "JFIF":
    case "JPEG":
    case "JPG":
    case "PJP":
    case "PJPEG":
    case "PNG":
    case "SVG":
    case "TIF":
    case "TIFF":
    case "WEBP":
      return "image";

    case "3GP":
    case "MOV":
    case "MP4":
    case "WEBM":
      return "video";

    case "MP3":
    case "WAV":
    case "OGG":
      return "audio";

    default:
      return "other";
  }
}
