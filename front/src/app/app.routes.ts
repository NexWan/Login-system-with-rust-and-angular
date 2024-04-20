import { Routes } from '@angular/router';
import { HomeComponent } from './home/home.component';
import { LoginComponent } from './login/login.component';
import { NotFoundComponent } from './not-found/not-found.component';

export const routes: Routes = [
    {path: '', component: LoginComponent,},
    {path: 'home', component: HomeComponent},
    {path: '**', component: NotFoundComponent}
];

/*Static routes of the front*/
