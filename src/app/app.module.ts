import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { MaterialModule } from './material/material.module';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { FrontPageComponent } from './components/front-page/front-page.component';
import { SettingsPageComponent } from './components/settings-page/settings-page.component';
import { AboutComponent } from './components/about/about.component';
import { PlayerSettingsComponent } from './components/settings-page/player-settings/player-settings.component';

@NgModule({
  declarations: [
    AppComponent,
    FrontPageComponent,
    SettingsPageComponent,
    AboutComponent,
    PlayerSettingsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MaterialModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
