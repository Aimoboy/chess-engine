import { Injectable } from '@angular/core';
import { PlayerConfig } from '../classes/PlayerConfig';

@Injectable({
  providedIn: 'root'
})
export class PlayerConfigService {

  private _whitePlayerConfig = PlayerConfig.defaultPlayerConfig();
  private _blackPlayerConfig = PlayerConfig.defaultPlayerConfig();

  constructor() { }

  public getWhitePlayerConfig() {
    return this._whitePlayerConfig;
  }

  public getBlackPlayerConfig() {
    return this._blackPlayerConfig;
  }
}
