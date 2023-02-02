import { Component } from '@angular/core';
import { PlayerConfig } from 'src/app/classes/PlayerConfig';
import { PlayerConfigService } from 'src/app/services/player-config.service';

@Component({
  selector: 'app-settings-page',
  templateUrl: './settings-page.component.html',
  styleUrls: ['./settings-page.component.scss']
})
export class SettingsPageComponent {

  constructor(public playerConfigService: PlayerConfigService) { }

}
