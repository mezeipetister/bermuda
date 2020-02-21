import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { Folder } from 'src/app/class/folder';
import { Document } from 'src/app/class/document';

@Component({
  selector: 'app-folder-detail',
  templateUrl: './folder-detail.component.html',
  styleUrls: ['./folder-detail.component.css']
})
export class FolderDetailComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  folder: Folder = new Folder();
  documents: Document[] = [];

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.http.get<Folder>("/folder/" + this.id).subscribe((val) => this.folder = val);
    this.http.get<Document[]>("/document/" + this.id + "/all").subscribe((val) => this.documents = val);
  }

  download(id) {
    let w = window.open();
    this.http.get("/file/" + id, { responseType: 'blob' }).subscribe((data) => {
      w.location.href = URL.createObjectURL(new Blob([data], { type: 'application/pdf' }));
    });
  }

}
