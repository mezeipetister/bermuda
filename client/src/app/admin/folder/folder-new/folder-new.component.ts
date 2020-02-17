import { Component, OnInit } from '@angular/core';
import { FolderNew, Folder } from 'src/app/class/folder';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

@Component({
  selector: 'app-folder-new',
  templateUrl: './folder-new.component.html',
  styleUrls: ['./folder-new.component.css']
})
export class FolderNewComponent implements OnInit {

  folder: FolderNew = new FolderNew();

  constructor(private http: HttpClient, private router: Router) { }

  ngOnInit() {

  }

  submit() {
    this.http.put<Folder>("/folder/new", this.folder).subscribe((val) => this.router.navigateByUrl("/folder/" + val.id));
  }

}
