import { ActivatedRoute, Router, RouterLink } from '@angular/router';
import {ChangeDetectorRef, Component, OnInit} from '@angular/core';
import { ConfigService } from '../config.service';
import {LoadingService} from "../loading.service";
import {NgIf} from "@angular/common";

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    NgIf
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent implements OnInit{
user: any;
password: any;
isLoading = false;

  constructor(private router:Router, private route: ActivatedRoute, private config: ConfigService, private loadingService:LoadingService, private cdr: ChangeDetectorRef) {
    this.loadingService.loading$.subscribe(loading => {
      this.isLoading = loading;
      this.cdr.detectChanges()
    })
  }
  ngOnInit(): void {
    if(sessionStorage.getItem('user') != null){
      this.router.navigate(['home']);
    }
  }

  async register(){
    this.router.navigate(['register']);
  }

  async login(){
    this.user = (<HTMLInputElement>document.getElementById('user')).value;
    this.password = (<HTMLInputElement>document.getElementById('password')).value;
    const res = await this.config.getUser(this.user, this.password);
    console.log(this.isLoading)
    res.subscribe((data)=>{
      this.isLoading = false;
      if(data.status == 'success'){ //We get the status of the response of the backend server
        this.router.navigate(['home']); // If the status is success we redirect to the home page
        sessionStorage.setItem('user', this.user); // We save the user in the session storage
      }else{
        alert(data.message); // If the status is not success we show the message of the response
      }
    });
  }

}
