import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { Model } from 'src/app/class/model';
import { Folder, FolderNewName, FolderNewDescription } from 'src/app/class/folder';

@Component({
  selector: 'app-folder-edit',
  templateUrl: './folder-edit.component.html',
  styleUrls: ['./folder-edit.component.css']
})
export class FolderEditComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  folder: Folder = new Folder();

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.http.get<Folder>("/folder/" + this.id).subscribe((val) => this.folder = val);
  }

  submitName() {
    this.http.post<Folder>("/folder/" + this.id + "/rename",
      new FolderNewName(this.folder.name)).subscribe((val) => {
        this.folder = val;
        alert("Sikeres módosítás!");
      });
  }

  submitDescription() {
    this.http.post<Folder>("/folder/" + this.id + "/redescription",
      new FolderNewDescription(this.folder.description)).subscribe((val) => {
        this.folder = val;
        alert("Sikeres módosítás!");
      });
  }

}
