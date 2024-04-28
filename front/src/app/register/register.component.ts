import { NgIf } from '@angular/common';
import { ChangeDetectorRef, Component } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { ConfigService } from '../config.service';
import { LoadingService } from '../loading.service';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [NgIf],
  templateUrl: './register.component.html',
  styleUrl: './register.component.scss'
})
export class RegisterComponent {
  isLoading = false;
  user: any;
  password: any;

  constructor(private router:Router, private route: ActivatedRoute, private config: ConfigService, private loadingService:LoadingService, private cdr: ChangeDetectorRef) {
    this.loadingService.loading$.subscribe(loading => {
      this.isLoading = loading;
      this.cdr.detectChanges()
    })
  }

  async register(){
    this.user = (<HTMLInputElement>document.getElementById('user')).value;
    this.password = (<HTMLInputElement>document.getElementById('password')).value;
    const res = await this.config.register(this.user, this.password);
    res.subscribe((data)=>{
      this.isLoading = false;
      if(data.status == 'success'){
        alert("User registered successfully");
        this.router.navigate(['home']);
        sessionStorage.setItem('user', this.user);
      }else{
        alert(data.message);
      }
    });
    console.log('Registering user');
  }

}
