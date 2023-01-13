import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FrontPageComponent } from './components/front-page/front-page.component';
import { SettingsPageComponent } from './components/settings-page/settings-page.component';

const routes: Routes = [
  { path: '', component: FrontPageComponent },
  { path: 'settings-page', component: SettingsPageComponent}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
