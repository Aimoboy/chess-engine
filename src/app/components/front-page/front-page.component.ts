import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { AboutComponent } from '../about/about.component';

@Component({
  selector: 'app-front-page',
  templateUrl: './front-page.component.html',
  styleUrls: ['./front-page.component.scss']
})
export class FrontPageComponent {

  public constructor(private _matDialog: MatDialog) {}

  onAboutClick() {
    this._matDialog.open(AboutComponent, {
      width: '400px',
      height: '400px'
    });
  }
}
