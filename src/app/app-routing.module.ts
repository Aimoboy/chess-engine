import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FrontPageComponent } from './components/front-page/front-page.component';
import { SettingsPageComponent } from './components/settings-page/settings-page.component';
import { BoardPageComponent } from './components/board-page/board-page.component';

const routes: Routes = [
  { path: '', component: FrontPageComponent },
  { path: 'settings-page', component: SettingsPageComponent},
  { path: 'board-page', component: BoardPageComponent}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
