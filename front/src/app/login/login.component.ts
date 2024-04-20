import { ActivatedRoute, Router } from '@angular/router';
import { Component, OnInit } from '@angular/core';
import { ConfigService } from '../config.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent implements OnInit{
user: any;
password: any;

  constructor(private router:Router, private route: ActivatedRoute, private config: ConfigService) { }
  ngOnInit(): void {
    if(sessionStorage.getItem('user') != null){
      this.router.navigate(['home']);
    }
  }

  login(){
    this.user = (<HTMLInputElement>document.getElementById('user')).value;
    this.password = (<HTMLInputElement>document.getElementById('password')).value;
    var res = this.config.getUser(this.user, this.password);
    res.subscribe((data)=>{
      if(data.status == 'success'){ //We get the status of the response of the backend server
        this.router.navigate(['home']); // If the status is success we redirect to the home page
        sessionStorage.setItem('user', this.user); // We save the user in the session storage
      }else{
        alert(data.message); // If the status is not success we show the message of the response
      }
    });
    console.log(this.user, this.password);
  }

}
