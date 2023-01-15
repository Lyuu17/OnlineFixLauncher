export const API_URL: string = "http://api.playfacapi.com/serverlist";

export declare interface ServerData {
  ServerName: string,
  ServerCapacity: number,
  MaxCapacity: number,
  ServerId: string,
  GameName: string;
};

export declare type ServerDataList = Array<ServerData>;