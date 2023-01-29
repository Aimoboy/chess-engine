import { Component } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-about',
  templateUrl: './about.component.html',
  styleUrls: ['./about.component.scss']
})
export class AboutComponent {

  constructor(private _dialogRef: MatDialogRef<AboutComponent>) { }

  public onCloseClick() {
    this._dialogRef.close();
  }

}
