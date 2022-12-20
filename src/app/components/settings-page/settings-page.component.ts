import { Component, ViewChild } from '@angular/core';
import { PlayerSettingsComponent } from './player-settings/player-settings.component';

@Component({
  selector: 'app-settings-page',
  templateUrl: './settings-page.component.html',
  styleUrls: ['./settings-page.component.scss']
})
export class SettingsPageComponent {

  public nextDisabled = true;

  @ViewChild('whiteSettings')
  public whiteSettings: PlayerSettingsComponent | null = null;

  @ViewChild('blackSettings')
  public blackSettings: PlayerSettingsComponent | null = null;

  

}
